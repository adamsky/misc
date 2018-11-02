//! Some style and color presets.
//TODO add a light style?

use imgui::*;


pub const DARK_COLOR: [f32; 4] = [0.01, 0.015, 0.02, 1.0];

pub fn dark_style() -> [StyleVar; 4] {
    [

        StyleVar::WindowRounding(2.0),
        //StyleVar::GrabRounding(5.3),
        StyleVar::FrameRounding(2.0),
        StyleVar::FrameBorderSize(2.0),
        //StyleVar::ItemSpacing.y(5.3),
        StyleVar::WindowBorderSize(0.0),

    ]
}

pub fn dark_colors() -> [(ImGuiCol, (f32, f32, f32, f32)); 38] {
    [
        (ImGuiCol::Text, (0.73333335, 0.73333335, 0.73333335, 1.00)),
        (ImGuiCol::TextDisabled, (0.37, 0.37, 0.38, 1.00)),
        (ImGuiCol::WindowBg, (0.23529413, 0.24705884, 0.25490198, 0.94)),
        (ImGuiCol::ChildBg, (0.23529413, 0.24705884, 0.25490198, 0.00)),
        (ImGuiCol::PopupBg, (0.23529413, 0.24705884, 0.25490198, 0.94)),
        (ImGuiCol::Border, (0.23333334, 0.23333334, 0.23333334, 0.45)),
        (ImGuiCol::BorderShadow, (0.15686275, 0.15686275, 0.15686275, 0.00)),
        (ImGuiCol::FrameBg, (0.16862746, 0.16862746, 0.16862746, 0.54)),
        (ImGuiCol::FrameBgHovered, (0.453125, 0.67578125, 0.99609375, 0.67)),
        (ImGuiCol::FrameBgActive, (0.47058827, 0.47058827, 0.47058827, 0.67)),
        (ImGuiCol::TitleBg, (0.23, 0.24, 0.25, 1.00)),
        (ImGuiCol::TitleBgCollapsed, (0.33, 0.34, 0.35, 1.00)),
        (ImGuiCol::TitleBgActive, (0.20, 0.21, 0.22, 1.0)),
        (ImGuiCol::MenuBarBg, (0.27058825, 0.28627452, 0.2901961, 0.80)),
        (ImGuiCol::ScrollbarBg, (0.23529413, 0.24705884, 0.25490198, 0.94)),
        (ImGuiCol::ScrollbarGrab, (0.39, 0.39, 0.39, 0.51)),
        (ImGuiCol::ScrollbarGrabHovered, (0.39, 0.39, 0.39, 1.00)),
        (ImGuiCol::ScrollbarGrabActive, (0.39, 0.39, 0.39, 0.91)),
        (ImGuiCol::CheckMark, (0.90, 0.90, 0.90, 0.83)),
        (ImGuiCol::SliderGrab, (0.70, 0.70, 0.70, 0.62)),
        (ImGuiCol::SliderGrabActive, (0.30, 0.30, 0.30, 0.84)),
        (ImGuiCol::Button, (0.33333334, 0.3529412, 0.36078432, 0.49)),
        (ImGuiCol::ButtonHovered, (0.21960786, 0.30980393, 0.41960788, 1.00)),
        (ImGuiCol::ButtonActive, (0.13725491, 0.19215688, 0.2627451, 1.00)),
        (ImGuiCol::Header, (0.33333334, 0.3529412, 0.36078432, 0.53)),
        (ImGuiCol::HeaderHovered, (0.453125, 0.67578125, 0.99609375, 0.67)),
        (ImGuiCol::HeaderActive, (0.47058827, 0.47058827, 0.47058827, 0.67)),
        (ImGuiCol::Separator, (0.31640625, 0.31640625, 0.31640625, 1.00)),
        (ImGuiCol::SeparatorHovered, (0.31640625, 0.31640625, 0.31640625, 1.00)),
        (ImGuiCol::SeparatorActive, (0.31640625, 0.31640625, 0.31640625, 1.00)),
        (ImGuiCol::ResizeGrip, (0.30, 0.30, 0.30, 0.55)),
        (ImGuiCol::ResizeGripHovered, (0.45, 0.45, 0.45, 0.60)),
        (ImGuiCol::ResizeGripActive, (0.50, 0.50, 0.50, 0.80)),
        (ImGuiCol::PlotLines, (0.61, 0.61, 0.61, 1.00)),
        (ImGuiCol::PlotLinesHovered, (1.00, 0.43, 0.35, 1.00)),
        (ImGuiCol::PlotHistogram, (0.90, 0.70, 0.00, 1.00)),
        (ImGuiCol::PlotHistogramHovered, (1.00, 0.60, 0.00, 1.00)),
        (ImGuiCol::TextSelectedBg, (0.18431373, 0.39607847, 0.79215693, 0.90)),
//        (ImGuiCol::CloseButton, (0.33333334, 0.3529412, 0.36078432, 0.49)),
//        (ImGuiCol::CloseButtonHovered, (0.21960786, 0.30980393, 0.41960788, 1.00)),
//        (ImGuiCol::CloseButtonActive, (0.13725491, 0.19215688, 0.2627451, 1.00)),
    ]
}