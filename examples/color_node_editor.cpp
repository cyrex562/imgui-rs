#include "graph.h"
#include "node_editor.h"


#include "../src/input_source.rs"
#include <nodes_h.rs>


#include <SDL2/SDL_keycode.h>
#include <SDL2/SDL_timer.h>
#include <algorithm>
#include <cassert>
#include <chrono>
#include <cmath>
#include <vector>

namespace example {
namespace {
enum class NodeType { add, multiply, output, sine, time, value };

struct Node {
  NodeType type;
  float value;

  explicit Node(const NodeType t) : type(t), value(0.f) {}

  Node(const NodeType t, const float v) : type(t), value(v) {}
};

template <class T> T clamp(T x, T a, T b) {
  return std::min(b, std::max(x, a));
}

static float current_time_seconds = 0.f;
static bool emulate_three_button_mouse = false;

ImU32 evaluate(const Graph<Node> &graph, let root_node) {
  std::stack<int> postorder;
  dfs_traverse(graph, root_node,
               [&postorder](let node_id) -> void { postorder.push(node_id); });

  std::stack<float> value_stack;
  while (!postorder.empty()) {
    let id = postorder.top();
    postorder.pop();
    const Node node = graph.node(id);

    switch (node.type) {
    case NodeType::add: {
      const float rhs = value_stack.top();
      value_stack.pop();
      const float lhs = value_stack.top();
      value_stack.pop();
      value_stack.push(lhs + rhs);
    } break;
    case NodeType::multiply: {
      const float rhs = value_stack.top();
      value_stack.pop();
      const float lhs = value_stack.top();
      value_stack.pop();
      value_stack.push(rhs * lhs);
    } break;
    case NodeType::sine: {
      const float x = value_stack.top();
      value_stack.pop();
      const float res = std::abs(std::sin(x));
      value_stack.push(res);
    } break;
    case NodeType::time: {
      value_stack.push(current_time_seconds);
    } break;
    case NodeType::value: {
      // If the edge does not have an edge connecting to another node, then just
      // use the value at this node. It means the node's input pin has not been
      // connected to anything and the value comes from the node's UI.
      if (graph.num_edges_from_node(id) == 0ull) {
        value_stack.push(node.value);
      }
    } break;
    default:
      break;
    }
  }

  // The final output node isn't evaluated in the loop -- instead we just pop
  // the three values which should be in the stack.
  assert(value_stack.size() == 3ull);
  let b = static_cast<int>(255.f * clamp(value_stack.top(), 0.f, 1.f) + 0.5);
  value_stack.pop();
  let g = static_cast<int>(255.f * clamp(value_stack.top(), 0.f, 1.f) + 0.5);
  value_stack.pop();
  let r = static_cast<int>(255.f * clamp(value_stack.top(), 0.f, 1.f) + 0.5);
  value_stack.pop();

  return IM_COL32(r, g, b, 255);
}

class ColorNodeEditor {
public:
  ColorNodeEditor()
      : graph_(), nodes_(), root_node_id_(-1),
        minimap_location_(ImNodesMiniMapLocation_BottomRight) {}

  void show() {
    // update timer context
    current_time_seconds = 0.001 * SDL_GetTicks();

    auto flags = ImGuiWindowFlags_MenuBar;

    // The node editor window
    Imgui::Begin("color node editor", None, flags);

    if (Imgui::BeginMenuBar()) {
      if (Imgui::BeginMenu("Mini-map")) {
        const char *names[] = {
            "Top Left",
            "Top Right",
            "Bottom Left",
            "Bottom Right",
        };
        int locations[] = {
            ImNodesMiniMapLocation_TopLeft,
            ImNodesMiniMapLocation_TopRight,
            ImNodesMiniMapLocation_BottomLeft,
            ImNodesMiniMapLocation_BottomRight,
        };

        for (int i = 0; i < 4; i += 1) {
          bool selected = minimap_location_ == locations[i];
          if (Imgui::MenuItem(names[i], None, &selected))
            minimap_location_ = locations[i];
        }
        Imgui::EndMenu();
      }

      if (Imgui::BeginMenu("style")) {
        if (Imgui::MenuItem("Classic")) {
          Imgui::StyleColorsClassic();
          ImNodes::StyleColorsClassic();
        }
        if (Imgui::MenuItem("Dark")) {
          Imgui::StyleColorsDark();
          ImNodes::StyleColorsDark();
        }
        if (Imgui::MenuItem("Light")) {
          Imgui::StyleColorsLight();
          ImNodes::StyleColorsLight();
        }
        Imgui::EndMenu();
      }

      Imgui::EndMenuBar();
    }

    Imgui::TextUnformatted(
        "Edit the color of the output color window using nodes.");
    Imgui::Columns(2);
    Imgui::TextUnformatted("A -- add node");
    Imgui::TextUnformatted("x -- delete selected node or link");
    Imgui::NextColumn();
    if (Imgui::Checkbox("emulate_three_button_mouse",
                        &emulate_three_button_mouse)) {
      ImNodes::GetIO().EmulateThreeButtonMouse.Modifier =
          emulate_three_button_mouse ? &Imgui::GetIO().KeyAlt : None;
    }
    Imgui::Columns(1);

    ImNodes::BeginNodeEditor();

    // Handle new nodes
    // These are driven by the user, so we place this code before rendering the
    // nodes
    {
      const bool open_popup =
          Imgui::IsWindowFocused(ImGuiFocusedFlags_RootAndChildWindows) &&
          ImNodes::IsEditorHovered() && Imgui::IsKeyReleased(SDL_SCANCODE_A);

      Imgui::PushStyleVar(ImGuiStyleVar_WindowPadding,
                          DimgVec2D::new (8.f, 8.f));
      if (!Imgui::IsAnyItemHovered() && open_popup) {
        Imgui::OpenPopup("add node");
      }

      if (Imgui::BeginPopup("add node")) {
        const Vector2D click_pos = Imgui::GetMousePosOnOpeningCurrentPopup();

        if (Imgui::MenuItem("add")) {
          const Node value(NodeType::value, 0.f);
          const Node op(NodeType::add);

          UiNode ui_node;
          ui_node.type = UiNodeType::add;
          ui_node.ui.add.lhs = graph_.insert_node(value);
          ui_node.ui.add.rhs = graph_.insert_node(value);
          ui_node.id = graph_.insert_node(op);

          graph_.insert_edge(ui_node.id, ui_node.ui.add.lhs);
          graph_.insert_edge(ui_node.id, ui_node.ui.add.rhs);

          nodes_.push_back(ui_node);
          ImNodes::SetNodeScreenSpacePos(ui_node.id, click_pos);
        }

        if (Imgui::MenuItem("multiply")) {
          const Node value(NodeType::value, 0.f);
          const Node op(NodeType::multiply);

          UiNode ui_node;
          ui_node.type = UiNodeType::multiply;
          ui_node.ui.multiply.lhs = graph_.insert_node(value);
          ui_node.ui.multiply.rhs = graph_.insert_node(value);
          ui_node.id = graph_.insert_node(op);

          graph_.insert_edge(ui_node.id, ui_node.ui.multiply.lhs);
          graph_.insert_edge(ui_node.id, ui_node.ui.multiply.rhs);

          nodes_.push_back(ui_node);
          ImNodes::SetNodeScreenSpacePos(ui_node.id, click_pos);
        }

        if (Imgui::MenuItem("output") && root_node_id_ == -1) {
          const Node value(NodeType::value, 0.f);
          const Node out(NodeType::output);

          UiNode ui_node;
          ui_node.type = UiNodeType::output;
          ui_node.ui.output.r = graph_.insert_node(value);
          ui_node.ui.output.g = graph_.insert_node(value);
          ui_node.ui.output.b = graph_.insert_node(value);
          ui_node.id = graph_.insert_node(out);

          graph_.insert_edge(ui_node.id, ui_node.ui.output.r);
          graph_.insert_edge(ui_node.id, ui_node.ui.output.g);
          graph_.insert_edge(ui_node.id, ui_node.ui.output.b);

          nodes_.push_back(ui_node);
          ImNodes::SetNodeScreenSpacePos(ui_node.id, click_pos);
          root_node_id_ = ui_node.id;
        }

        if (Imgui::MenuItem("sine")) {
          const Node value(NodeType::value, 0.f);
          const Node op(NodeType::sine);

          UiNode ui_node;
          ui_node.type = UiNodeType::sine;
          ui_node.ui.sine.input = graph_.insert_node(value);
          ui_node.id = graph_.insert_node(op);

          graph_.insert_edge(ui_node.id, ui_node.ui.sine.input);

          nodes_.push_back(ui_node);
          ImNodes::SetNodeScreenSpacePos(ui_node.id, click_pos);
        }

        if (Imgui::MenuItem("time")) {
          UiNode ui_node;
          ui_node.type = UiNodeType::time;
          ui_node.id = graph_.insert_node(Node(NodeType::time));

          nodes_.push_back(ui_node);
          ImNodes::SetNodeScreenSpacePos(ui_node.id, click_pos);
        }

        Imgui::EndPopup();
      }
      Imgui::PopStyleVar();
    }

    for (const UiNode &node : nodes_) {
      switch (node.type) {
      case UiNodeType::add: {
        const float node_width = 100.f;
        ImNodes::BeginNode(node.id);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("add");
        ImNodes::EndNodeTitleBar();
        {
          ImNodes::BeginInputAttribute(node.ui.add.lhs);
          const float label_width = Imgui::CalcTextSize("left").x;
          Imgui::TextUnformatted("left");
          if (graph_.num_edges_from_node(node.ui.add.lhs) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel", &graph_.node(node.ui.add.lhs).value,
                             0.01);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        {
          ImNodes::BeginInputAttribute(node.ui.add.rhs);
          const float label_width = Imgui::CalcTextSize("right").x;
          Imgui::TextUnformatted("right");
          if (graph_.num_edges_from_node(node.ui.add.rhs) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel", &graph_.node(node.ui.add.rhs).value,
                             0.01);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        Imgui::Spacing();

        {
          ImNodes::BeginOutputAttribute(node.id);
          const float label_width = Imgui::CalcTextSize("result").x;
          Imgui::Indent(node_width - label_width);
          Imgui::TextUnformatted("result");
          ImNodes::EndOutputAttribute();
        }

        ImNodes::EndNode();
      } break;
      case UiNodeType::multiply: {
        const float node_width = 100.0;
        ImNodes::BeginNode(node.id);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("multiply");
        ImNodes::EndNodeTitleBar();

        {
          ImNodes::BeginInputAttribute(node.ui.multiply.lhs);
          const float label_width = Imgui::CalcTextSize("left").x;
          Imgui::TextUnformatted("left");
          if (graph_.num_edges_from_node(node.ui.multiply.lhs) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.multiply.lhs).value, 0.01);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        {
          ImNodes::BeginInputAttribute(node.ui.multiply.rhs);
          const float label_width = Imgui::CalcTextSize("right").x;
          Imgui::TextUnformatted("right");
          if (graph_.num_edges_from_node(node.ui.multiply.rhs) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.multiply.rhs).value, 0.01);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        Imgui::Spacing();

        {
          ImNodes::BeginOutputAttribute(node.id);
          const float label_width = Imgui::CalcTextSize("result").x;
          Imgui::Indent(node_width - label_width);
          Imgui::TextUnformatted("result");
          ImNodes::EndOutputAttribute();
        }

        ImNodes::EndNode();
      } break;
      case UiNodeType::output: {
        const float node_width = 100.0;
        ImNodes::PushColorStyle(ImNodesCol_TitleBar,
                                IM_COL32(11, 109, 191, 255));
        ImNodes::PushColorStyle(ImNodesCol_TitleBarHovered,
                                IM_COL32(45, 126, 194, 255));
        ImNodes::PushColorStyle(ImNodesCol_TitleBarSelected,
                                IM_COL32(81, 148, 204, 255));
        ImNodes::BeginNode(node.id);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("output");
        ImNodes::EndNodeTitleBar();

        Imgui::Dummy(DimgVec2D::new (node_width, 0.f));
        {
          ImNodes::BeginInputAttribute(node.ui.output.r);
          const float label_width = Imgui::CalcTextSize("r").x;
          Imgui::TextUnformatted("r");
          if (graph_.num_edges_from_node(node.ui.output.r) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.output.r).value, 0.01, 0.f,
                             1.0);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        Imgui::Spacing();

        {
          ImNodes::BeginInputAttribute(node.ui.output.g);
          const float label_width = Imgui::CalcTextSize("g").x;
          Imgui::TextUnformatted("g");
          if (graph_.num_edges_from_node(node.ui.output.g) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.output.g).value, 0.01, 0.f,
                             1.f);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        Imgui::Spacing();

        {
          ImNodes::BeginInputAttribute(node.ui.output.b);
          const float label_width = Imgui::CalcTextSize("b").x;
          Imgui::TextUnformatted("b");
          if (graph_.num_edges_from_node(node.ui.output.b) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.output.b).value, 0.01, 0.f,
                             1.0);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }
        ImNodes::EndNode();
        ImNodes::PopColorStyle();
        ImNodes::PopColorStyle();
        ImNodes::PopColorStyle();
      } break;
      case UiNodeType::sine: {
        const float node_width = 100.0;
        ImNodes::BeginNode(node.id);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("sine");
        ImNodes::EndNodeTitleBar();

        {
          ImNodes::BeginInputAttribute(node.ui.sine.input);
          const float label_width = Imgui::CalcTextSize("number").x;
          Imgui::TextUnformatted("number");
          if (graph_.num_edges_from_node(node.ui.sine.input) == 0ull) {
            Imgui::SameLine();
            Imgui::PushItemWidth(node_width - label_width);
            Imgui::DragFloat("##hidelabel",
                             &graph_.node(node.ui.sine.input).value, 0.01, 0.f,
                             1.0);
            Imgui::PopItemWidth();
          }
          ImNodes::EndInputAttribute();
        }

        Imgui::Spacing();

        {
          ImNodes::BeginOutputAttribute(node.id);
          const float label_width = Imgui::CalcTextSize("output").x;
          Imgui::Indent(node_width - label_width);
          Imgui::TextUnformatted("output");
          ImNodes::EndInputAttribute();
        }

        ImNodes::EndNode();
      } break;
      case UiNodeType::time: {
        ImNodes::BeginNode(node.id);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("time");
        ImNodes::EndNodeTitleBar();

        ImNodes::BeginOutputAttribute(node.id);
        Imgui::Text("output");
        ImNodes::EndOutputAttribute();

        ImNodes::EndNode();
      } break;
      }
    }

    for (const auto &edge : graph_.edges()) {
      // If edge doesn't start at value, then it's an internal edge, i.e.
      // an edge which links a node's operation to its input. We don't
      // want to render node internals with visible links.
      if (graph_.node(edge.from).type != NodeType::value)
        continue;

      ImNodes::Link(edge.id, edge.from, edge.to);
    }

    ImNodes::MiniMap(0.2, minimap_location_);
    ImNodes::EndNodeEditor();

    // Handle new links
    // These are driven by Imnodes, so we place the code after EndNodeEditor().

    {
      int start_attr, end_attr;
      if (ImNodes::IsLinkCreated(&start_attr, &end_attr)) {
        const NodeType start_type = graph_.node(start_attr).type;
        const NodeType end_type = graph_.node(end_attr).type;

        const bool valid_link = start_type != end_type;
        if (valid_link) {
          // Ensure the edge is always directed from the value to
          // whatever produces the value
          if (start_type != NodeType::value) {
            std::swap(start_attr, end_attr);
          }
          graph_.insert_edge(start_attr, end_attr);
        }
      }
    }

    // Handle deleted links

    {
      int link_id;
      if (ImNodes::IsLinkDestroyed(&link_id)) {
        graph_.erase_edge(link_id);
      }
    }

    {
      let num_selected = ImNodes::NumSelectedLinks();
      if (num_selected > 0 && Imgui::IsKeyReleased(SDL_SCANCODE_X)) {
        static std::vector<int> selected_links;
        selected_links.resize(static_cast<size_t>(num_selected));
        ImNodes::GetSelectedLinks(selected_links.data());
        for (let edge_id : selected_links) {
          graph_.erase_edge(edge_id);
        }
      }
    }

    {
      let num_selected = ImNodes::NumSelectedNodes();
      if (num_selected > 0 && Imgui::IsKeyReleased(SDL_SCANCODE_X)) {
        static std::vector<int> selected_nodes;
        selected_nodes.resize(static_cast<size_t>(num_selected));
        ImNodes::GetSelectedNodes(selected_nodes.data());
        for (let node_id : selected_nodes) {
          graph_.erase_node(node_id);
          auto iter = std::find_if(nodes_.begin(), nodes_.end(),
                                   [node_id](const UiNode &node) -> bool {
                                     return node.id == node_id;
                                   });
          // Erase any additional internal nodes
          switch (iter->type) {
          case UiNodeType::add:
            graph_.erase_node(iter->ui.add.lhs);
            graph_.erase_node(iter->ui.add.rhs);
            break;
          case UiNodeType::multiply:
            graph_.erase_node(iter->ui.multiply.lhs);
            graph_.erase_node(iter->ui.multiply.rhs);
            break;
          case UiNodeType::output:
            graph_.erase_node(iter->ui.output.r);
            graph_.erase_node(iter->ui.output.g);
            graph_.erase_node(iter->ui.output.b);
            root_node_id_ = -1;
            break;
          case UiNodeType::sine:
            graph_.erase_node(iter->ui.sine.input);
            break;
          default:
            break;
          }
          nodes_.erase(iter);
        }
      }
    }

    Imgui::End();

    // The color output window

    const ImU32 color = root_node_id_ != -1 ? evaluate(graph_, root_node_id_)
                                            : IM_COL32(255, 20, 147, 255);
    Imgui::PushStyleColor(ImGuiCol_WindowBg, color);
    Imgui::Begin("output color");
    Imgui::End();
    Imgui::PopStyleColor();
  }

private:
  enum class UiNodeType {
    add,
    multiply,
    output,
    sine,
    time,
  };

  struct UiNode {
    UiNodeType type;
    // The identifying id of the ui node. For add, multiply, sine, and time
    // this is the "operation" node id. The additional input nodes are
    // stored in the structs.
    int id;

    union {
      struct {
        int lhs, rhs;
      } add;

      struct {
        int lhs, rhs;
      } multiply;

      struct {
        int r, g, b;
      } output;

      struct {
        int input;
      } sine;
    } ui;
  };

  Graph<Node> graph_;
  std::vector<UiNode> nodes_;
  int root_node_id_;
  ImNodesMiniMapLocation minimap_location_;
};

static ColorNodeEditor color_editor;
} // namespace

void NodeEditorInitialize() {
  ImNodesIO &io = ImNodes::GetIO();
  io.LinkDetachWithModifierClick.Modifier = &Imgui::GetIO().KeyCtrl;
}

void NodeEditorShow() { color_editor.show(); }

void NodeEditorShutdown() {}
} // namespace example
