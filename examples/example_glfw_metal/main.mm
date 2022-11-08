// Dear ImGui: standalone example application for GLFW + Metal, using programmable pipeline
// (GLFW is a cross-platform general purpose library for handling windows, inputs, OpenGL/Vulkan/Metal graphics context creation, etc.)
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_metal.h"
#include <stdio.h>

#define GLFW_INCLUDE_NONE
#define GLFW_EXPOSE_NATIVE_COCOA
#include <GLFW/glfw3.h>
#include <GLFW/glfw3native.h>

#import <Metal/Metal.h>
#import <QuartzCore/QuartzCore.h>

static void glfw_error_callback(int error, const char* description)
{
    fprintf(stderr, "Glfw Error %d: %s\n", error, description);
}

int main(int, char**)
{
    // Setup Dear ImGui context
    IMGUI_CHECKVERSION();
    Imgui::CreateContext();
    ImGuiIO& io = Imgui::GetIO(); (void)io;
    //io.config_flags |= ImGuiConfigFlags_NavEnableKeyboard;  // Enable Keyboard Controls
    //io.config_flags |= ImGuiConfigFlags_NavEnableGamepad;   // Enable Gamepad Controls
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;        // Enable Docking
    io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;      // Enable Multi-viewport / Platform windows

    // Setup style
    Imgui::StyleColorsDark();
    //ImGui::StyleColorsClassic();

    // When viewports are enabled we tweak window_rounding/WindowBg so platform windows can look identical to regular ones.
    ImGuiStyle& style = Imgui::GetStyle();
    if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
    {
        style.WindowRounding = 0.0;
        style.Colors[ImGuiCol_WindowBg].w = 1.0;
    }

    // Load fonts
    // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use ImGui::PushFont()/PopFont() to select them.
    // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
    // - If the file cannot be loaded, the function will return None. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
    // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
    // - Read 'docs/FONTS.txt' for more instructions and details.
    // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
    //io.fonts->add_font_default();
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0);
    //ImFont* font = io.fonts->AddFontFromFileTTF("c:\\windows\\fonts\\ArialUni.ttf", 18.0, None, io.fonts->get_glyph_ranges_japanese());
    //IM_ASSERT(font != None);

    // Setup window
    glfwSetErrorCallback(glfw_error_callback);
    if (!glfwInit())
        return 1;

    // Create window with graphics context
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    GLFWwindow* window = glfwCreateWindow(1280, 720, "Dear ImGui GLFW+Metal example", None, None);
    if (window == None)
        return 1;

    id <MTLDevice> device = MTLCreateSystemDefaultDevice();
    id <MTLCommandQueue> commandQueue = [device newCommandQueue];

    // Setup Platform/Renderer backends
    ImGui_ImplGlfw_InitForOther(window, true);
    ImGui_ImplMetal_Init(device);

    NSWindow *nswin = glfwGetCocoaWindow(window);
    CAMetalLayer *layer = [CAMetalLayer layer];
    layer.device = device;
    layer.pixelFormat = MTLPixelFormatBGRA8Unorm;
    nswin.contentView.layer = layer;
    nswin.contentView.wantsLayer = YES;

    MTLRenderPassDescriptor *renderPassDescriptor = [MTLRenderPassDescriptor new];

    // Our state
    bool show_demo_window = true;
    bool show_another_window = false;
    float clear_color[4] = {0.45, 0.55, 0.60, 1.00};

    // Main loop
    while (!glfwWindowShouldClose(window))
    {
        @autoreleasepool
        {
            // Poll and handle events (inputs, window resize, etc.)
            // You can read the io.want_capture_mouse, io.want_capture_keyboard flags to tell if dear imgui wants to use your inputs.
            // - When io.want_capture_mouse is true, do not dispatch mouse input data to your main application, or clear/overwrite your copy of the mouse data.
            // - When io.want_capture_keyboard is true, do not dispatch keyboard input data to your main application, or clear/overwrite your copy of the keyboard data.
            // Generally you may always pass all inputs to dear imgui, and hide them from your application based on those two flags.
            glfwPollEvents();

            int width, height;
            glfwGetFramebufferSize(window, &width, &height);
            layer.drawableSize = CGSizeMake(width, height);
            id<CAMetalDrawable> drawable = [layer nextDrawable];

            id<MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];
            renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(clear_color[0] * clear_color[3], clear_color[1] * clear_color[3], clear_color[2] * clear_color[3], clear_color[3]);
            renderPassDescriptor.colorAttachments[0].texture = drawable.texture;
            renderPassDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
            renderPassDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
            id <MTLRenderCommandEncoder> renderEncoder = [commandBuffer renderCommandEncoderWithDescriptor:renderPassDescriptor];
            [renderEncoder pushDebugGroup:@"ImGui demo"];

            // Start the Dear ImGui frame
            ImGui_ImplMetal_NewFrame(renderPassDescriptor);
            ImGui_ImplGlfw_NewFrame();
            Imgui::NewFrame();

            // 1. Show the big demo window (Most of the sample code is in ImGui::ShowDemoWindow()! You can browse its code to learn more about Dear ImGui!).
            if (show_demo_window)
                Imgui::ShowDemoWindow(&show_demo_window);

            // 2. Show a simple window that we create ourselves. We use a Begin/End pair to created a named window.
            {
                static float f = 0.0;
                static int counter = 0;

                Imgui::Begin("Hello, world!");                          // Create a window called "Hello, world!" and append into it.

                Imgui::Text("This is some useful text.");               // Display some text (you can use a format strings too)
                Imgui::Checkbox("Demo window", &show_demo_window);      // Edit bools storing our window open/close state
                Imgui::Checkbox("Another window", &show_another_window);

                Imgui::SliderFloat("float", &f, 0.0, 1.0);            // Edit 1 float using a slider from 0.0 to 1.0
                Imgui::ColorEdit3("clear color", (float*)&clear_color); // Edit 3 floats representing a color

                if (Imgui::Button("Button"))                            // Buttons return true when clicked (most widgets return true when edited/activated)
                    counter += 1;
                Imgui::SameLine();
                Imgui::Text("counter = %d", counter);

                Imgui::Text("Application average %.3 ms/frame (%.1 FPS)", 1000.0 / Imgui::GetIO().Framerate, Imgui::GetIO().Framerate);
                Imgui::End();
            }

            // 3. Show another simple window.
            if (show_another_window)
            {
                Imgui::Begin("Another window", &show_another_window);   // Pass a pointer to our bool variable (the window will have a closing button that will clear the bool when clicked)
                Imgui::Text("Hello from another window!");
                if (Imgui::Button("Close Me"))
                    show_another_window = false;
                Imgui::End();
            }

            // Rendering
            Imgui::Render();
            ImGui_ImplMetal_RenderDrawData(Imgui::GetDrawData(), commandBuffer, renderEncoder);

            // update and Render additional Platform windows
            if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
            {
                Imgui::UpdatePlatformWindows();
                Imgui::RenderPlatformWindowsDefault();
            }

            [renderEncoder popDebugGroup];
            [renderEncoder endEncoding];

            [commandBuffer presentDrawable:drawable];
            [commandBuffer commit];
        }
    }

    // Cleanup
    ImGui_ImplMetal_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    Imgui::DestroyContext();

    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}
