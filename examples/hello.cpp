#include "node_editor.h"
#include <nodes_h.rs>
#include "../src/defines.rs"

namespace example
{
namespace
{
class HelloWorldNodeEditor
{
public:
    void show()
    {
        ImGui::Begin("simple node editor");

        ImNodes::BeginNodeEditor();
        ImNodes::BeginNode(1);

        ImNodes::BeginNodeTitleBar();
        ImGui::TextUnformatted("simple node :)");
        ImNodes::EndNodeTitleBar();

        ImNodes::BeginInputAttribute(2);
        ImGui::Text("input");
        ImNodes::EndInputAttribute();

        ImNodes::BeginOutputAttribute(3);
        ImGui::Indent(40);
        ImGui::Text("output");
        ImNodes::EndOutputAttribute();

        ImNodes::EndNode();
        ImNodes::EndNodeEditor();

        ImGui::End();
    }
};

static HelloWorldNodeEditor editor;
} // namespace

void NodeEditorInitialize() { ImNodes::SetNodeGridSpacePos(1, DimgVec2D::new(200.0, 200.0)); }

void NodeEditorShow() { editor.show(); }

void NodeEditorShutdown() {}

} // namespace example
