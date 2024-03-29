// Dear ImGui: standalone example application for GLFW + OpenGL2, using legacy fixed pipeline
// (GLFW is a cross-platform general purpose library for handling windows, inputs, OpenGL/Vulkan/Metal graphics context creation, etc.)
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// **DO NOT USE THIS CODE IF YOUR CODE/ENGINE IS USING MODERN OPENGL (SHADERS, VBO, VAO, etc.)**
// **Prefer using the code in the example_glfw_opengl2/ folder**
// See imgui_impl_glfw.cpp for details.

// #include "imgui.h"
// #include "imgui_impl_glfw.h"
// #include "imgui_impl_opengl2.h"
// #include <stdio.h>
#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#endif
// #include <GLFW/glfw3.h>

// [Win32] Our example includes a copy of glfw3.lib pre-compiled with VS2010 to maximize ease of testing and compatibility with old VS compilers.
// To link with VS2010-era libraries, VS2015+ requires linking with legacy_stdio_definitions.lib, which we do using this pragma.
// Your own project should not be affected, as you are likely to link with a newer binary of GLFW that is adequate for your version of Visual Studio.
#if defined(_MSC_VER) && (_MSC_VER >= 1900) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS)
#pragma comment(lib, "legacy_stdio_definitions")
#endif

static void glfw_error_callback(int error, const char* description)
{
    fprintf(stderr, "Glfw Error %d: %s\n", error, description);
}

int main(int, char**)
{
    // Setup window
    glfwSetErrorCallback(glfw_error_callback);
    if (!glfwInit())
        return 1;
    GLFWwindow* window = glfwCreateWindow(1280, 720, "Dear ImGui GLFW+OpenGL2 example", None, None);
    if (window == None)
        return 1;
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1); // Enable vsync

    // Setup Dear ImGui context
    IMGUI_CHECKVERSION();
    Imgui::CreateContext();
    ImGuiIO& io = Imgui::GetIO(); (void)io;
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;       // Enable Keyboard Controls
    //io.config_flags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;           // Enable Docking
    io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;         // Enable Multi-viewport / Platform windows
    //io.config_viewports_no_auto_merge = true;
    //io.config_viewports_no_task_bar_icon = true;

    // Setup Dear ImGui style
    Imgui::StyleColorsDark();
    //ImGui::StyleColorsClassic();

    // When viewports are enabled we tweak window_rounding/WindowBg so platform windows can look identical to regular ones.
    ImGuiStyle& style = Imgui::GetStyle();
    if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
    {
        style.WindowRounding = 0.0;
        style.Colors[ImGuiCol_WindowBg].w = 1.0;
    }

    // Setup Platform/Renderer backends
    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL2_Init();

    // Load fonts
    // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use ImGui::PushFont()/PopFont() to select them.
    // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
    // - If the file cannot be loaded, the function will return None. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
    // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
    // - Read 'docs/FONTS.md' for more instructions and details.
    // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
    //io.fonts->add_font_default();
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0);
    //io.fonts->AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0);
    //ImFont* font = io.fonts->AddFontFromFileTTF("c:\\windows\\fonts\\ArialUni.ttf", 18.0, None, io.fonts->get_glyph_ranges_japanese());
    //IM_ASSERT(font != None);

    // Our state
    bool show_demo_window = true;
    bool show_another_window = false;
    Vector4D clear_color = Vector4D(0.45, 0.55, 0.60, 1.00);

    // Main loop
    while (!glfwWindowShouldClose(window))
    {
        // Poll and handle events (inputs, window resize, etc.)
        // You can read the io.want_capture_mouse, io.want_capture_keyboard flags to tell if dear imgui wants to use your inputs.
        // - When io.want_capture_mouse is true, do not dispatch mouse input data to your main application, or clear/overwrite your copy of the mouse data.
        // - When io.want_capture_keyboard is true, do not dispatch keyboard input data to your main application, or clear/overwrite your copy of the keyboard data.
        // Generally you may always pass all inputs to dear imgui, and hide them from your application based on those two flags.
        glfwPollEvents();

        // Start the Dear ImGui frame
        ImGui_ImplOpenGL2_NewFrame();
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
        int display_w, display_h;
        glfwGetFramebufferSize(window, &display_w, &display_h);
        glViewport(0, 0, display_w, display_h);
        glClearColor(clear_color.x * clear_color.w, clear_color.y * clear_color.w, clear_color.z * clear_color.w, clear_color.w);
        glClear(GL_COLOR_BUFFER_BIT);

        // If you are using this code with non-legacy OpenGL header/contexts (which you should not, prefer using imgui_impl_opengl3.cpp!!),
        // you may need to backup/reset/restore other state, e.g. for current shader using the commented lines below.
        //GLint last_program;
        //glGetIntegerv(GL_CURRENT_PROGRAM, &last_program);
        //glUseProgram(0);
        ImGui_ImplOpenGL2_RenderDrawData(Imgui::GetDrawData());
        //glUseProgram(last_program);

        // update and Render additional Platform windows
        // (Platform functions may change the current OpenGL context, so we save/restore it to make it easier to paste this code elsewhere.
        //  For this specific demo app we could also call glfwMakeContextCurrent(window) directly)
        if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
        {
            GLFWwindow* backup_current_context = glfwGetCurrentContext();
            Imgui::UpdatePlatformWindows();
            Imgui::RenderPlatformWindowsDefault();
            glfwMakeContextCurrent(backup_current_context);
        }

        glfwSwapBuffers(window);
    }

    // Cleanup
    ImGui_ImplOpenGL2_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    Imgui::DestroyContext();

    glfwDestroyWindow(window);
    glfwTerminate();

    return 0;
}
