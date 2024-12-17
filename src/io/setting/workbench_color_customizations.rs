use serde::Serialize;

use crate::model::{Color, ColorMap, HexStr};

#[derive(Serialize)]
pub(super) struct WorkbenchColorCustomizations {
    #[serde(rename = "focusBorder")]
    focus_border: HexStr,
    #[serde(rename = "foreground")]
    foreground: HexStr,
    #[serde(rename = "descriptionForeground")]
    description_foreground: HexStr,
    #[serde(rename = "errorForeground")]
    error_foreground: HexStr,
    #[serde(rename = "textLink.foreground")]
    text_link_foreground: HexStr,
    #[serde(rename = "textLink.activeForeground")]
    text_link_active_foreground: HexStr,
    #[serde(rename = "textBlockQuote.background")]
    text_block_quote_background: HexStr,
    #[serde(rename = "textBlockQuote.border")]
    text_block_quote_border: HexStr,
    #[serde(rename = "textCodeBlock.background")]
    text_code_block_background: HexStr,
    #[serde(rename = "textPreformat.foreground")]
    text_preformat_foreground: HexStr,
    #[serde(rename = "textSeparator.foreground")]
    text_separator_foreground: HexStr,
    #[serde(rename = "button.background")]
    button_background: HexStr,
    #[serde(rename = "button.foreground")]
    button_foreground: HexStr,
    #[serde(rename = "button.hoverBackground")]
    button_hover_background: HexStr,
    #[serde(rename = "button.secondaryBackground")]
    button_secondary_background: HexStr,
    #[serde(rename = "button.secondaryForeground")]
    button_secondary_foreground: HexStr,
    #[serde(rename = "button.secondaryHoverBackground")]
    button_secondary_hover_background: HexStr,
    #[serde(rename = "checkbox.background")]
    checkbox_background: HexStr,
    #[serde(rename = "checkbox.border")]
    checkbox_border: HexStr,
    #[serde(rename = "dropdown.background")]
    dropdown_background: HexStr,
    #[serde(rename = "dropdown.border")]
    dropdown_border: HexStr,
    #[serde(rename = "dropdown.foreground")]
    dropdown_foreground: HexStr,
    #[serde(rename = "dropdown.listBackground")]
    dropdown_list_background: HexStr,
    #[serde(rename = "input.background")]
    input_background: HexStr,
    #[serde(rename = "input.border")]
    input_border: HexStr,
    #[serde(rename = "input.foreground")]
    input_foreground: HexStr,
    #[serde(rename = "input.placeholderForeground")]
    input_placeholder_foreground: HexStr,
    #[serde(rename = "badge.foreground")]
    badge_foreground: HexStr,
    #[serde(rename = "badge.background")]
    badge_background: HexStr,
    #[serde(rename = "progressBar.background")]
    progress_bar_background: HexStr,
    #[serde(rename = "titleBar.activeForeground")]
    title_bar_active_foreground: HexStr,
    #[serde(rename = "titleBar.activeBackground")]
    title_bar_active_background: HexStr,
    #[serde(rename = "titleBar.inactiveForeground")]
    title_bar_inactive_foreground: HexStr,
    #[serde(rename = "titleBar.inactiveBackground")]
    title_bar_inactive_background: HexStr,
    #[serde(rename = "titleBar.border")]
    title_bar_border: HexStr,
    #[serde(rename = "activityBar.foreground")]
    activity_bar_foreground: HexStr,
    #[serde(rename = "activityBar.inactiveForeground")]
    activity_bar_inactive_foreground: HexStr,
    #[serde(rename = "activityBar.background")]
    activity_bar_background: HexStr,
    #[serde(rename = "activityBarBadge.foreground")]
    activity_bar_badge_foreground: HexStr,
    #[serde(rename = "activityBarBadge.background")]
    activity_bar_badge_background: HexStr,
    #[serde(rename = "activityBar.activeBorder")]
    activity_bar_active_border: HexStr,
    #[serde(rename = "activityBar.border")]
    activity_bar_border: HexStr,
    #[serde(rename = "sideBar.foreground")]
    side_bar_foreground: HexStr,
    #[serde(rename = "sideBar.background")]
    side_bar_background: HexStr,
    #[serde(rename = "sideBar.border")]
    side_bar_border: HexStr,
    #[serde(rename = "sideBarTitle.foreground")]
    side_bar_title_foreground: HexStr,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    side_bar_section_header_foreground: HexStr,
    #[serde(rename = "sideBarSectionHeader.background")]
    side_bar_section_header_background: HexStr,
    #[serde(rename = "sideBarSectionHeader.border")]
    side_bar_section_header_border: HexStr,
    #[serde(rename = "list.hoverForeground")]
    list_hover_foreground: HexStr,
    #[serde(rename = "list.inactiveSelectionForeground")]
    list_inactive_selection_foreground: HexStr,
    #[serde(rename = "list.activeSelectionForeground")]
    list_active_selection_foreground: HexStr,
    #[serde(rename = "list.hoverBackground")]
    list_hover_background: HexStr,
    #[serde(rename = "list.inactiveSelectionBackground")]
    list_inactive_selection_background: HexStr,
    #[serde(rename = "list.activeSelectionBackground")]
    list_active_selection_background: HexStr,
    #[serde(rename = "list.inactiveFocusBackground")]
    list_inactive_focus_background: HexStr,
    #[serde(rename = "list.focusBackground")]
    list_focus_background: HexStr,
    #[serde(rename = "tree.indentGuidesStroke")]
    tree_indent_guides_stroke: HexStr,
    #[serde(rename = "notificationCenterHeader.foreground")]
    notification_center_header_foreground: HexStr,
    #[serde(rename = "notificationCenterHeader.background")]
    notification_center_header_background: HexStr,
    #[serde(rename = "notifications.foreground")]
    notifications_foreground: HexStr,
    #[serde(rename = "notifications.background")]
    notifications_background: HexStr,
    #[serde(rename = "notifications.border")]
    notifications_border: HexStr,
    #[serde(rename = "notificationsErrorIcon.foreground")]
    notifications_error_icon_foreground: HexStr,
    #[serde(rename = "notificationsWarningIcon.foreground")]
    notifications_warning_icon_foreground: HexStr,
    #[serde(rename = "notificationsInfoIcon.foreground")]
    notifications_info_icon_foreground: HexStr,
    #[serde(rename = "pickerGroup.border")]
    picker_group_border: HexStr,
    #[serde(rename = "pickerGroup.foreground")]
    picker_group_foreground: HexStr,
    #[serde(rename = "quickInput.background")]
    quick_input_background: HexStr,
    #[serde(rename = "quickInput.foreground")]
    quick_input_foreground: HexStr,
    #[serde(rename = "statusBar.foreground")]
    status_bar_foreground: HexStr,
    #[serde(rename = "statusBar.background")]
    status_bar_background: HexStr,
    #[serde(rename = "statusBar.border")]
    status_bar_border: HexStr,
    #[serde(rename = "statusBar.noFolderBackground")]
    status_bar_no_folder_background: HexStr,
    #[serde(rename = "statusBar.debuggingBackground")]
    status_bar_debugging_background: HexStr,
    #[serde(rename = "statusBar.debuggingForeground")]
    status_bar_debugging_foreground: HexStr,
    #[serde(rename = "statusBarItem.prominentBackground")]
    status_bar_item_prominent_background: HexStr,
    #[serde(rename = "statusBarItem.remoteForeground")]
    status_bar_item_remote_foreground: HexStr,
    #[serde(rename = "statusBarItem.remoteBackground")]
    status_bar_item_remote_background: HexStr,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    editor_group_header_tabs_background: HexStr,
    #[serde(rename = "editorGroupHeader.tabsBorder")]
    editor_group_header_tabs_border: HexStr,
    #[serde(rename = "editorGroup.border")]
    editor_group_border: HexStr,
    #[serde(rename = "tab.activeForeground")]
    tab_active_foreground: HexStr,
    #[serde(rename = "tab.inactiveForeground")]
    tab_inactive_foreground: HexStr,
    #[serde(rename = "tab.inactiveBackground")]
    tab_inactive_background: HexStr,
    #[serde(rename = "tab.activeBackground")]
    tab_active_background: HexStr,
    #[serde(rename = "tab.hoverBackground")]
    tab_hover_background: HexStr,
    #[serde(rename = "tab.unfocusedHoverBackground")]
    tab_unfocused_hover_background: HexStr,
    #[serde(rename = "tab.border")]
    tab_border: HexStr,
    #[serde(rename = "tab.unfocusedActiveBorderTop")]
    tab_unfocused_active_border_top: HexStr,
    #[serde(rename = "tab.activeBorder")]
    tab_active_border: HexStr,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    tab_unfocused_active_border: HexStr,
    #[serde(rename = "tab.activeBorderTop")]
    tab_active_border_top: HexStr,
    #[serde(rename = "breadcrumb.foreground")]
    breadcrumb_foreground: HexStr,
    #[serde(rename = "breadcrumb.focusForeground")]
    breadcrumb_focus_foreground: HexStr,
    #[serde(rename = "breadcrumb.activeSelectionForeground")]
    breadcrumb_active_selection_foreground: HexStr,
    #[serde(rename = "breadcrumbPicker.background")]
    breadcrumb_picker_background: HexStr,
    #[serde(rename = "editor.foreground")]
    editor_foreground: HexStr,
    #[serde(rename = "editor.background")]
    editor_background: HexStr,
    #[serde(rename = "editorWidget.background")]
    editor_widget_background: HexStr,
    #[serde(rename = "editor.foldBackground")]
    editor_fold_background: HexStr,
    #[serde(rename = "editor.lineHighlightBackground")]
    editor_line_highlight_background: HexStr,
    #[serde(rename = "editorLineNumber.foreground")]
    editor_line_number_foreground: HexStr,
    #[serde(rename = "editorLineNumber.activeForeground")]
    editor_line_number_active_foreground: HexStr,
    #[serde(rename = "editorIndentGuide.background")]
    editor_indent_guide_background: HexStr,
    #[serde(rename = "editorIndentGuide.activeBackground")]
    editor_indent_guide_active_background: HexStr,
    #[serde(rename = "editorWhitespace.foreground")]
    editor_whitespace_foreground: HexStr,
    #[serde(rename = "editorCursor.foreground")]
    editor_cursor_foreground: HexStr,
    #[serde(rename = "editorError.foreground")]
    editor_error_foreground: HexStr,
    #[serde(rename = "editorWarning.foreground")]
    editor_warning_foreground: HexStr,
    #[serde(rename = "editor.findMatchBackground")]
    editor_find_match_background: HexStr,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    editor_find_match_highlight_background: HexStr,
    #[serde(rename = "editor.linkedEditingBackground")]
    editor_linked_editing_background: HexStr,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    editor_inactive_selection_background: HexStr,
    #[serde(rename = "editor.selectionBackground")]
    editor_selection_background: HexStr,
    #[serde(rename = "editor.selectionHighlightBackground")]
    editor_selection_highlight_background: HexStr,
    #[serde(rename = "editor.selectionHighlightBorder")]
    editor_selection_highlight_border: HexStr,
    #[serde(rename = "editor.wordHighlightBackground")]
    editor_word_highlight_background: HexStr,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    editor_word_highlight_strong_background: HexStr,
    #[serde(rename = "editor.wordHighlightBorder")]
    editor_word_highlight_border: HexStr,
    #[serde(rename = "editor.wordHighlightStrongBorder")]
    editor_word_highlight_strong_border: HexStr,
    #[serde(rename = "editorBracketMatch.background")]
    editor_bracket_match_background: HexStr,
    #[serde(rename = "editorBracketMatch.border")]
    editor_bracket_match_border: HexStr,
    #[serde(rename = "editorGutter.modifiedBackground")]
    editor_gutter_modified_background: HexStr,
    #[serde(rename = "editorGutter.addedBackground")]
    editor_gutter_added_background: HexStr,
    #[serde(rename = "editorGutter.deletedBackground")]
    editor_gutter_deleted_background: HexStr,
    #[serde(rename = "scrollbar.shadow")]
    scrollbar_shadow: HexStr,
    #[serde(rename = "scrollbarSlider.background")]
    scrollbar_slider_background: HexStr,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    scrollbar_slider_hover_background: HexStr,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    scrollbar_slider_active_background: HexStr,
    #[serde(rename = "editorOverviewRuler.border")]
    editor_overview_ruler_border: HexStr,
    #[serde(rename = "panel.background")]
    panel_background: HexStr,
    #[serde(rename = "panel.border")]
    panel_border: HexStr,
    #[serde(rename = "panelTitle.activeBorder")]
    panel_title_active_border: HexStr,
    #[serde(rename = "panelTitle.activeForeground")]
    panel_title_active_foreground: HexStr,
    #[serde(rename = "panelTitle.inactiveForeground")]
    panel_title_inactive_foreground: HexStr,
    #[serde(rename = "panelInput.border")]
    panel_input_border: HexStr,
    #[serde(rename = "terminal.foreground")]
    terminal_foreground: HexStr,
    #[serde(rename = "terminal.tab.activeBorder")]
    terminal_tab_active_border: HexStr,
    #[serde(rename = "terminalCursor.background")]
    terminal_cursor_background: HexStr,
    #[serde(rename = "terminalCursor.foreground")]
    terminal_cursor_foreground: HexStr,
    #[serde(rename = "terminal.ansiBrightWhite")]
    terminal_ansi_bright_white: HexStr,
    #[serde(rename = "terminal.ansiWhite")]
    terminal_ansi_white: HexStr,
    #[serde(rename = "terminal.ansiBrightBlack")]
    terminal_ansi_bright_black: HexStr,
    #[serde(rename = "terminal.ansiBlack")]
    terminal_ansi_black: HexStr,
    #[serde(rename = "terminal.ansiBlue")]
    terminal_ansi_blue: HexStr,
    #[serde(rename = "terminal.ansiBrightBlue")]
    terminal_ansi_bright_blue: HexStr,
    #[serde(rename = "terminal.ansiGreen")]
    terminal_ansi_green: HexStr,
    #[serde(rename = "terminal.ansiBrightGreen")]
    terminal_ansi_bright_green: HexStr,
    #[serde(rename = "terminal.ansiCyan")]
    terminal_ansi_cyan: HexStr,
    #[serde(rename = "terminal.ansiBrightCyan")]
    terminal_ansi_bright_cyan: HexStr,
    #[serde(rename = "terminal.ansiRed")]
    terminal_ansi_red: HexStr,
    #[serde(rename = "terminal.ansiBrightRed")]
    terminal_ansi_bright_red: HexStr,
    #[serde(rename = "terminal.ansiMagenta")]
    terminal_ansi_magenta: HexStr,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    terminal_ansi_bright_magenta: HexStr,
    #[serde(rename = "terminal.ansiYellow")]
    terminal_ansi_yellow: HexStr,
    #[serde(rename = "terminal.ansiBrightYellow")]
    terminal_ansi_bright_yellow: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground1")]
    editor_bracket_highlight_foreground1: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground2")]
    editor_bracket_highlight_foreground2: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground3")]
    editor_bracket_highlight_foreground3: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground4")]
    editor_bracket_highlight_foreground4: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground5")]
    editor_bracket_highlight_foreground5: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground6")]
    editor_bracket_highlight_foreground6: HexStr,
    #[serde(rename = "debugToolBar.background")]
    debug_tool_bar_background: HexStr,
    #[serde(rename = "editor.stackFrameHighlightBackground")]
    editor_stack_frame_highlight_background: HexStr,
    #[serde(rename = "editor.focusedStackFrameHighlightBackground")]
    editor_focused_stack_frame_highlight_background: HexStr,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    peek_view_editor_match_highlight_background: HexStr,
    #[serde(rename = "peekViewResult.matchHighlightBackground")]
    peek_view_result_match_highlight_background: HexStr,
    #[serde(rename = "peekViewEditor.background")]
    peek_view_editor_background: HexStr,
    #[serde(rename = "peekViewResult.background")]
    peek_view_result_background: HexStr,
    #[serde(rename = "settings.headerForeground")]
    settings_header_foreground: HexStr,
    #[serde(rename = "settings.modifiedItemIndicator")]
    settings_modified_item_indicator: HexStr,
    #[serde(rename = "welcomePage.buttonBackground")]
    welcome_page_button_background: HexStr,
    #[serde(rename = "welcomePage.buttonHoverBackground")]
    welcome_page_button_hover_background: HexStr,
}

impl WorkbenchColorCustomizations {
    pub(super) fn new(color_map: &ColorMap<super::HexColors>, ui_gray: super::HexColors) -> Self {
        Self {
            focus_border: color_map[Color::Blue][2],
            foreground: ui_gray[3],
            description_foreground: color_map[Color::Gray][2],
            error_foreground: color_map[Color::Red][3],

            text_link_foreground: color_map[Color::Blue][3],
            text_link_active_foreground: color_map[Color::Blue][3],
            text_block_quote_background: color_map[Color::Bg][0],
            text_block_quote_border: color_map[Color::Gray][1],
            text_code_block_background: color_map[Color::Bg][0],
            text_preformat_foreground: color_map[Color::Gray][3],
            text_separator_foreground: color_map[Color::Gray][1],

            button_background: color_map[Color::Bg][0],
            button_foreground: ui_gray[4],
            button_hover_background: color_map[Color::Bg][1],

            button_secondary_background: color_map[Color::Bg][1],
            button_secondary_foreground: ui_gray[3],
            button_secondary_hover_background: color_map[Color::Bg][2],

            checkbox_background: color_map[Color::Bg][1],
            checkbox_border: color_map[Color::Gray][3],

            dropdown_background: color_map[Color::Bg][1],
            dropdown_border: color_map[Color::Gray][3],
            dropdown_foreground: ui_gray[3],
            dropdown_list_background: color_map[Color::Bg][0],

            input_background: color_map[Color::Bg][1],
            input_border: color_map[Color::Gray][3],
            input_foreground: ui_gray[3],
            input_placeholder_foreground: ui_gray[2],

            badge_foreground: ui_gray[3],
            badge_background: color_map[Color::Bg][1],

            progress_bar_background: color_map[Color::Blue][2],

            title_bar_active_foreground: ui_gray[3],
            title_bar_active_background: color_map[Color::Bg][2],
            title_bar_inactive_foreground: ui_gray[2],
            title_bar_inactive_background: color_map[Color::Bg][1],
            title_bar_border: color_map[Color::Gray][3],

            activity_bar_foreground: ui_gray[3],
            activity_bar_inactive_foreground: color_map[Color::Gray][2],
            activity_bar_background: color_map[Color::Bg][2],
            activity_bar_badge_foreground: ui_gray[3],
            activity_bar_badge_background: color_map[Color::Bg][2],
            activity_bar_active_border: color_map[Color::Gray][2],
            activity_bar_border: color_map[Color::Gray][3],

            side_bar_foreground: ui_gray[3],
            side_bar_background: color_map[Color::Bg][3],
            side_bar_border: color_map[Color::Gray][0],
            side_bar_title_foreground: ui_gray[3],
            side_bar_section_header_foreground: ui_gray[3],
            side_bar_section_header_background: color_map[Color::Bg][3],
            side_bar_section_header_border: color_map[Color::Gray][3],

            list_hover_foreground: ui_gray[3],
            list_inactive_selection_foreground: ui_gray[3],
            list_active_selection_foreground: ui_gray[3],
            list_hover_background: color_map[Color::Bg][3],
            list_inactive_selection_background: color_map[Color::Bg][2],
            list_active_selection_background: color_map[Color::Bg][1],
            list_inactive_focus_background: color_map[Color::Bg][0],
            list_focus_background: color_map[Color::Bg][4],

            tree_indent_guides_stroke: color_map[Color::Gray][1],

            notification_center_header_foreground: ui_gray[2],
            notification_center_header_background: color_map[Color::Bg][0],
            notifications_foreground: ui_gray[2],
            notifications_background: color_map[Color::Bg][2],
            notifications_border: color_map[Color::Gray][3],
            notifications_error_icon_foreground: color_map[Color::Red][0],
            notifications_warning_icon_foreground: color_map[Color::Yellow][2],
            notifications_info_icon_foreground: color_map[Color::Blue][3],

            picker_group_border: color_map[Color::Gray][1],
            picker_group_foreground: ui_gray[2],

            quick_input_background: color_map[Color::Bg][0],
            quick_input_foreground: ui_gray[2],

            status_bar_foreground: ui_gray[2],
            status_bar_background: color_map[Color::Bg][2],
            status_bar_border: color_map[Color::Gray][2],
            status_bar_no_folder_background: color_map[Color::Bg][0],
            status_bar_debugging_background: color_map[Color::Bg][2],
            status_bar_debugging_foreground: color_map[Color::Gray][2],
            status_bar_item_prominent_background: color_map[Color::Gray][3],
            status_bar_item_remote_foreground: ui_gray[3],
            status_bar_item_remote_background: color_map[Color::Bg][0],

            editor_group_header_tabs_background: color_map[Color::Bg][1],
            editor_group_header_tabs_border: color_map[Color::Gray][2],
            editor_group_border: color_map[Color::Gray][2],

            tab_active_foreground: ui_gray[2],
            tab_inactive_foreground: ui_gray[3],
            tab_inactive_background: color_map[Color::Bg][2],
            tab_active_background: color_map[Color::Bg][4],
            tab_hover_background: color_map[Color::Bg][3],
            tab_unfocused_hover_background: color_map[Color::Bg][4],
            tab_border: color_map[Color::Gray][2],
            tab_unfocused_active_border_top: color_map[Color::Gray][2],
            tab_active_border: color_map[Color::Gray][0],
            tab_unfocused_active_border: color_map[Color::Gray][0],
            tab_active_border_top: color_map[Color::Gray][1],

            breadcrumb_foreground: ui_gray[3],
            breadcrumb_focus_foreground: ui_gray[2],
            breadcrumb_active_selection_foreground: color_map[Color::Gray][4],
            breadcrumb_picker_background: color_map[Color::Bg][2],

            editor_foreground: ui_gray[2],
            editor_background: color_map[Color::Bg][2],
            editor_widget_background: color_map[Color::Bg][3],
            editor_fold_background: color_map[Color::Bg][4].alpha(0.5),
            editor_line_highlight_background: color_map[Color::Bg][4],
            editor_line_number_foreground: ui_gray[1],
            editor_line_number_active_foreground: ui_gray[2],
            editor_indent_guide_background: color_map[Color::Bg][0],
            editor_indent_guide_active_background: color_map[Color::Bg][1],
            editor_whitespace_foreground: ui_gray[1],
            editor_cursor_foreground: ui_gray[3],
            editor_error_foreground: color_map[Color::Red][3],
            editor_warning_foreground: color_map[Color::Yellow][3],

            editor_find_match_background: color_map[Color::Bg][4],
            editor_find_match_highlight_background: color_map[Color::Bg][0].alpha(0.5),
            editor_linked_editing_background: color_map[Color::Bg][4],
            editor_inactive_selection_background: color_map[Color::Bg][4].alpha(0.5),
            editor_selection_background: color_map[Color::Bg][4],
            editor_selection_highlight_background: color_map[Color::Bg][4].alpha(0.5),
            editor_selection_highlight_border: color_map[Color::Gray][0],
            editor_word_highlight_background: color_map[Color::Bg][4].alpha(0.5),
            editor_word_highlight_strong_background: color_map[Color::Bg][4].alpha(0.5),
            editor_word_highlight_border: color_map[Color::Gray][0],
            editor_word_highlight_strong_border: color_map[Color::Gray][0],
            editor_bracket_match_background: color_map[Color::Bg][4],
            editor_bracket_match_border: color_map[Color::Gray][0],

            editor_gutter_modified_background: color_map[Color::Blue][2],
            editor_gutter_added_background: color_map[Color::Green][1],
            editor_gutter_deleted_background: color_map[Color::Red][2],

            scrollbar_shadow: color_map[Color::Bg][4],
            scrollbar_slider_background: color_map[Color::Bg][2],
            scrollbar_slider_hover_background: color_map[Color::Bg][1],
            scrollbar_slider_active_background: color_map[Color::Bg][0],
            editor_overview_ruler_border: color_map[Color::Gray][2],

            panel_background: color_map[Color::Bg][2],
            panel_border: color_map[Color::Gray][2],
            panel_title_active_border: color_map[Color::Gray][0],
            panel_title_active_foreground: color_map[Color::Gray][0],
            panel_title_inactive_foreground: color_map[Color::Gray][4],
            panel_input_border: color_map[Color::Gray][0],

            terminal_foreground: ui_gray[3],
            terminal_tab_active_border: color_map[Color::Gray][4],
            terminal_cursor_background: color_map[Color::Bg][2],
            terminal_cursor_foreground: color_map[Color::Gray][2],

            terminal_ansi_bright_white: ui_gray[4],
            terminal_ansi_white: ui_gray[4],
            terminal_ansi_bright_black: ui_gray[0],
            terminal_ansi_black: ui_gray[0],
            terminal_ansi_blue: color_map[Color::Blue][2],
            terminal_ansi_bright_blue: color_map[Color::Blue][2],
            terminal_ansi_green: color_map[Color::Green][2],
            terminal_ansi_bright_green: color_map[Color::Green][2],
            terminal_ansi_cyan: color_map[Color::Blue][2],
            terminal_ansi_bright_cyan: color_map[Color::Blue][2],
            terminal_ansi_red: color_map[Color::Red][2],
            terminal_ansi_bright_red: color_map[Color::Red][2],
            terminal_ansi_magenta: color_map[Color::Purple][2],
            terminal_ansi_bright_magenta: color_map[Color::Purple][2],
            terminal_ansi_yellow: color_map[Color::Yellow][2],
            terminal_ansi_bright_yellow: color_map[Color::Yellow][2],

            editor_bracket_highlight_foreground1: color_map[Color::Blue][3],
            editor_bracket_highlight_foreground2: color_map[Color::Orange][3],
            editor_bracket_highlight_foreground3: color_map[Color::Purple][3],
            editor_bracket_highlight_foreground4: color_map[Color::Blue][3],
            editor_bracket_highlight_foreground5: color_map[Color::Orange][3],
            editor_bracket_highlight_foreground6: color_map[Color::Purple][3],

            debug_tool_bar_background: color_map[Color::Bg][2],
            editor_stack_frame_highlight_background: color_map[Color::Bg][3],
            editor_focused_stack_frame_highlight_background: color_map[Color::Bg][3],

            peek_view_editor_match_highlight_background: color_map[Color::Bg][3],
            peek_view_result_match_highlight_background: color_map[Color::Bg][4],
            peek_view_editor_background: color_map[Color::Bg][0],
            peek_view_result_background: color_map[Color::Bg][1],

            settings_header_foreground: ui_gray[2],
            settings_modified_item_indicator: color_map[Color::Blue][2],
            welcome_page_button_background: color_map[Color::Bg][0],
            welcome_page_button_hover_background: color_map[Color::Bg][1],
        }
    }
}
