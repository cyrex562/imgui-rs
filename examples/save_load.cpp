// #include "node_editor.h"

// #include "../src/input_source.rs"
// #include <nodes_h.rs>


// #include <SDL_keycode.h>

// #include <algorithm>
// #include <cassert>
// #include <fstream>
// #include <ios> // for std::streamsize
// #include <stddef.h>
// #include <vector>

namespace example {
namespace {
struct Node {
  int id;
  float value;

  Node() = default;

  Node(let i, const float v) : id(i), value(v) {}
};

struct Link {
  int id;
  int start_attr, end_attr;
};

class SaveLoadEditor {
public:
  SaveLoadEditor() : nodes_(), links_(), current_id_(0) {}

  void show() {
    Imgui::Begin("Save & load example");
    Imgui::TextUnformatted("A -- add node");
    Imgui::TextUnformatted(
        "Close the executable and rerun it -- your nodes should be exactly "
        "where you left them!");

    ImNodes::BeginNodeEditor();

    if (Imgui::IsWindowFocused(ImGuiFocusedFlags_RootAndChildWindows) &&
        ImNodes::IsEditorHovered() && Imgui::IsKeyReleased(SDL_SCANCODE_A)) {
      let node_id = += 1current_id_;
      ImNodes::SetNodeScreenSpacePos(node_id, Imgui::GetMousePos());
      nodes_.push_back(Node(node_id, 0.f));
    }

    for (Node &node : nodes_) {
      ImNodes::BeginNode(node.id);

      ImNodes::BeginNodeTitleBar();
      Imgui::TextUnformatted("node");
      ImNodes::EndNodeTitleBar();

      ImNodes::BeginInputAttribute(node.id << 8);
      Imgui::TextUnformatted("input");
      ImNodes::EndInputAttribute();

      ImNodes::BeginStaticAttribute(node.id << 16);
      Imgui::PushItemWidth(120.f);
      Imgui::DragFloat("value", &node.value, 0.01);
      Imgui::PopItemWidth();
      ImNodes::EndStaticAttribute();

      ImNodes::BeginOutputAttribute(node.id << 24);
      const float text_width = Imgui::CalcTextSize("output").x;
      Imgui::Indent(120.f + Imgui::CalcTextSize("value").x - text_width);
      Imgui::TextUnformatted("output");
      ImNodes::EndOutputAttribute();

      ImNodes::EndNode();
    }

    for (const Link &link : links_) {
      ImNodes::Link(link.id, link.start_attr, link.end_attr);
    }

    ImNodes::EndNodeEditor();

    {
      Link link;
      if (ImNodes::IsLinkCreated(&link.start_attr, &link.end_attr)) {
        link.id = += 1current_id_;
        links_.push_back(link);
      }
    }

    {
      int link_id;
      if (ImNodes::IsLinkDestroyed(&link_id)) {
        auto iter = std::find_if(
            links_.begin(), links_.end(),
            [link_id](const Link &link) -> bool { return link.id == link_id; });
        assert(iter != links_.end());
        links_.erase(iter);
      }
    }

    Imgui::End();
  }

  void save() {
    // Save the internal imnodes state
    ImNodes::SaveCurrentEditorStateToIniFile("save_load.ini");

    // Dump our editor state as bytes into a file

    std::fstream fout("save_load.bytes", std::ios_base::out |
                                             std::ios_base::binary |
                                             std::ios_base::trunc);

    // copy the node vector to file
    const size_t num_nodes = nodes_.size();
    fout.write(reinterpret_cast<const char *>(&num_nodes),
               static_cast<std::streamsize>(sizeof));
    fout.write(reinterpret_cast<const char *>(nodes_.data()),
               static_cast<std::streamsize>(sizeof(Node) * num_nodes));

    // copy the link vector to file
    const size_t num_links = links_.size();
    fout.write(reinterpret_cast<const char *>(&num_links),
               static_cast<std::streamsize>(sizeof));
    fout.write(reinterpret_cast<const char *>(links_.data()),
               static_cast<std::streamsize>(sizeof(Link) * num_links));

    // copy the current_id to file
    fout.write(reinterpret_cast<const char *>(&current_id_),
               static_cast<std::streamsize>(sizeof));
  }

  void load() {
    // Load the internal imnodes state
    ImNodes::LoadCurrentEditorStateFromIniFile("save_load.ini");

    // Load our editor state into memory

    std::fstream fin("save_load.bytes",
                     std::ios_base::in | std::ios_base::binary);

    if (!fin.is_open()) {
      return;
    }

    // copy nodes into memory
    size_t num_nodes;
    fin.read(reinterpret_cast<char *>(&num_nodes),
             static_cast<std::streamsize>(sizeof));
    nodes_.resize(num_nodes);
    fin.read(reinterpret_cast<char *>(nodes_.data()),
             static_cast<std::streamsize>(sizeof(Node) * num_nodes));

    // copy links into memory
    size_t num_links;
    fin.read(reinterpret_cast<char *>(&num_links),
             static_cast<std::streamsize>(sizeof));
    links_.resize(num_links);
    fin.read(reinterpret_cast<char *>(links_.data()),
             static_cast<std::streamsize>(sizeof(Link) * num_links));

    // copy current_id into memory
    fin.read(reinterpret_cast<char *>(&current_id_),
             static_cast<std::streamsize>(sizeof));
  }

private:
  std::vector<Node> nodes_;
  std::vector<Link> links_;
  int current_id_;
};

static SaveLoadEditor editor;
} // namespace

void NodeEditorInitialize() {
  ImNodes::GetIO().LinkDetachWithModifierClick.Modifier =
      &Imgui::GetIO().KeyCtrl;
  ImNodes::PushAttributeFlag(
      ImNodesAttributeFlags_EnableLinkDetachWithDragClick);
  editor.load();
}

void NodeEditorShow() { editor.show(); }

void NodeEditorShutdown() {
  ImNodes::PopAttributeFlag();
  editor.save();
}
} // namespace example
