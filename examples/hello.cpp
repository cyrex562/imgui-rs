// #include "node_editor.h"
// #include <nodes_h.rs>
// #include "../src/input_source.rs"

namespace example
{
namespace
{
class HelloWorldNodeEditor
{
public:
    void show()
    {
        Imgui::Begin("simple node editor");

        ImNodes::BeginNodeEditor();
        ImNodes::BeginNode(1);

        ImNodes::BeginNodeTitleBar();
        Imgui::TextUnformatted("simple node :)");
        ImNodes::EndNodeTitleBar();

        ImNodes::BeginInputAttribute(2);
        Imgui::Text("input");
        ImNodes::EndInputAttribute();

        ImNodes::BeginOutputAttribute(3);
        Imgui::Indent(40);
        Imgui::Text("output");
        ImNodes::EndOutputAttribute();

        ImNodes::EndNode();
        ImNodes::EndNodeEditor();

        Imgui::End();
    }
};

static HelloWorldNodeEditor editor;
} // namespace

void NodeEditorInitialize() { ImNodes::SetNodeGridSpacePos(1, DimgVec2D::new(200.0, 200.0)); }

void NodeEditorShow() { editor.show(); }

void NodeEditorShutdown() {}

} // namespace example
