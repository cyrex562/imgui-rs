// Dear ImGui: standalone example application for GLUT/FreeGLUT + OpenGL2, using legacy fixed pipeline
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// !!! GLUT/FreeGLUT IS OBSOLETE PREHISTORIC SOFTWARE. Using GLUT is not recommended unless you really miss the 90's. !!!
// !!! If someone or something is teaching you GLUT today, you are being abused. Please show some resistance. !!!
// !!! Nowadays, prefer using GLFW or SDL instead!

// On windows, you can install Freeglut using vcpkg:
//   git clone https://github.com/Microsoft/vcpkg
//   cd vcpkg
//   bootstrap - vcpkg.bat
//   vcpkg install freeglut --triplet=x86-windows   ; for win32
//   vcpkg install freeglut --triplet=x64-windows   ; for win64
//   vcpkg integrate install                        ; register include and libs in Visual Studio

// #include "imgui.h"
// #include "imgui_impl_glut.h"
// #include "imgui_impl_opengl2.h"
#ifdef __APPLE__
    #include <GLUT/glut.h>
#else
    #include <GL/freeglut.h>
#endif

#ifdef _MSC_VER
#pragma warning (disable: 4505) // unreferenced local function has been removed
#endif

// Our state
static bool show_demo_window = true;
static bool show_another_window = false;
static Vector4D clear_color = Vector4D(0.45, 0.55, 0.60, 1.00);

void my_display_code()
{
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
}

void glut_display_func()
{
    // Start the Dear ImGui frame
    ImGui_ImplOpenGL2_NewFrame();
    ImGui_ImplGLUT_NewFrame();

    my_display_code();

    // Rendering
    Imgui::Render();
    ImGuiIO& io = Imgui::GetIO();
    glViewport(0, 0, (GLsizei)io.DisplaySize.x, (GLsizei)io.DisplaySize.y);
    glClearColor(clear_color.x * clear_color.w, clear_color.y * clear_color.w, clear_color.z * clear_color.w, clear_color.w);
    glClear(GL_COLOR_BUFFER_BIT);
    //glUseProgram(0); // You may want this if using this code in an OpenGL 3+ context where shaders may be bound, but prefer using the GL3+ code.
    ImGui_ImplOpenGL2_RenderDrawData(Imgui::GetDrawData());

    glutSwapBuffers();
    glutPostRedisplay();
}

// You can read the io.want_capture_mouse, io.want_capture_keyboard flags to tell if dear imgui wants to use your inputs.
// - When io.want_capture_mouse is true, do not dispatch mouse input data to your main application, or clear/overwrite your copy of the mouse data.
// - When io.want_capture_keyboard is true, do not dispatch keyboard input data to your main application, or clear/overwrite your copy of the keyboard data.
// Generally you may always pass all inputs to dear imgui, and hide them from your application based on those two flags.

int main(int argc, char** argv)
{
    // Create GLUT window
    glutInit(&argc, argv);
#ifdef __FREEGLUT_EXT_H__
    glutSetOption(GLUT_ACTION_ON_WINDOW_CLOSE, GLUT_ACTION_GLUTMAINLOOP_RETURNS);
#endif
    glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_MULTISAMPLE);
    glutInitWindowSize(1280, 720);
    glutCreateWindow("Dear ImGui GLUT+OpenGL2 Example");

    // Setup GLUT display function
    // We will also call ImGui_ImplGLUT_InstallFuncs() to get all the other functions installed for us,
    // otherwise it is possible to install our own functions and call the imgui_impl_glut.h functions ourselves.
    glutDisplayFunc(glut_display_func);

    // Setup Dear ImGui context
    IMGUI_CHECKVERSION();
    Imgui::CreateContext();
    ImGuiIO& io = Imgui::GetIO(); (void)io;
    //io.config_flags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;           // Enable Docking

    // Setup Dear ImGui style
    Imgui::StyleColorsDark();
    //ImGui::StyleColorsClassic();

    // Setup Platform/Renderer backends
    // FIXME: Consider reworking this example to install our own GLUT funcs + forward calls ImGui_ImplGLUT_XXX ones, instead of using ImGui_ImplGLUT_InstallFuncs().
    ImGui_ImplGLUT_Init();
    ImGui_ImplGLUT_InstallFuncs();
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

    glutMainLoop();

    // Cleanup
    ImGui_ImplOpenGL2_Shutdown();
    ImGui_ImplGLUT_Shutdown();
    Imgui::DestroyContext();

    return 0;
}
