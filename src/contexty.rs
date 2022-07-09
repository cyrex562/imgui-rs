// typedef void (*ImGuiContextHookCallback)(ImGuiContext* ctx, ImGuiContextHook* hook);
type DimgContextHookCallback = fn(ctx: &mut DimgContext, hook: &mut DimgContextHook);
