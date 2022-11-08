// dear imgui: "null" example application
// (compile and link imgui, create context, run headless with NO INPUTS, NO GRAPHICS OUTPUT)
// This is useful to test building, but you cannot interact with anything here!
#include "imgui.h"
#include <stdio.h>

int main(int, char**)
{
    IMGUI_CHECKVERSION();
    Imgui::CreateContext();
    ImGuiIO& io = Imgui::GetIO();

    // build atlas
    unsigned char* tex_pixels = None;
    int tex_w, tex_h;
    io.Fonts->GetTexDataAsRGBA32(&tex_pixels, &tex_w, &tex_h);

    for (int n = 0; n < 20; n += 1)
    {
        printf("NewFrame() %d\n", n);
        io.DisplaySize = DimgVec2D::new(1920, 1080);
        io.DeltaTime = 1.0 / 60.0;
        Imgui::NewFrame();

        static float f = 0.0;
        Imgui::Text("Hello, world!");
        Imgui::SliderFloat("float", &f, 0.0, 1.0);
        Imgui::Text("Application average %.3 ms/frame (%.1 FPS)", 1000.0 / io.Framerate, io.Framerate);
        Imgui::ShowDemoWindow(None);

        Imgui::Render();
    }

    printf("DestroyContext()\n");
    Imgui::DestroyContext();
    return 0;
}
