use crate::{Context, Viewport};
use crate::platform::PlatformImeData;

// static void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport* viewport, ImGuiPlatformImeData* data)
pub fn set_platform_ime_data_fn_default_impl(g: &mut Context, viewport: &mut Viewport, data: &mut PlatformImeData)
{
    // Notify OS Input Method Editor of text input position
    HWND hwnd = (HWND)viewport->platform_handleRaw;
#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if (hwnd == 0)
        hwnd = (HWND)GetIO().ImeWindowHandle;

    if (hwnd == 0)
        return;

    ::ImmAssociateContextEx(hwnd, None, data->WantVisible ? IACE_DEFAULT : 0);

    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data->InputPos.x - viewport.pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data->InputPos.y - viewport.pos.y);
        ::ImmSetCandidateWindow(himc, &candidate_form);
        ::ImmReleaseContext(hwnd, himc);
    }
}
