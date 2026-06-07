use crate::agent::skill::skill::*;

pub struct BuiltInSkills;

impl BuiltInSkills {
    pub fn search_in_app() -> Skill {
        Skill::new("search_in_app", "Search in App",
            "Type a search query and submit it. Use when the task says 'search for' or 'find' in any app.",
            SkillCategory::Input)
            .save_steps(5)
            .with_parameters(vec![
                SkillParameter::new("query", true).with_description("The search query to type")
            ])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Check for search bar"),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Search")])
                    .with_description("Tap search icon/bar").optional(),
                SkillStep::new("input_text").with_params(vec![("text", "{query}")])
                    .with_description("Type query"),
                SkillStep::new("system_key").with_params(vec![("key", "enter")])
                    .with_description("Submit search"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "2000")])
                    .with_description("Wait for results"),
            ])
            .with_fallback("The search bar should have '{query}' typed. Just press enter or submit.")
    }

    pub fn submit_form() -> Skill {
        Skill::new("submit_form", "Submit Form",
            "Find and tap the Send/Submit/Save button. Use after typing a message or filling a form.",
            SkillCategory::Input)
            .save_steps(4)
            .with_triggers(vec!["submit", "send message", "press send"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Look for submit button"),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Send|Submit|Save|Post|Done")])
                    .with_description("Tap submit button"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "2000")])
                    .with_description("Wait for submission"),
            ])
            .with_fallback("Find and tap the Send, Submit, or Save button on screen.")
    }

    pub fn dismiss_popup() -> Skill {
        Skill::new("dismiss_popup", "Dismiss Popup",
            "Close common popups, dialogs, and overlays. Use when a dialog blocks the task.",
            SkillCategory::Dismiss)
            .save_steps(3)
            .with_triggers(vec!["dismiss", "close popup", "close dialog"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Identify popup type"),
                SkillStep::new("find_and_tap").with_params(vec![("text", "OK")])
                    .with_description("Tap OK").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Got it")])
                    .with_description("Tap Got it").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Close")])
                    .with_description("Tap Close").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Dismiss")])
                    .with_description("Tap Dismiss").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Not now")])
                    .with_description("Tap Not now").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Cancel")])
                    .with_description("Tap Cancel").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Skip")])
                    .with_description("Tap Skip").optional(),
            ])
            .with_fallback("A popup or dialog is blocking. Find and tap the close/dismiss button.")
    }

    pub fn scroll_and_read() -> Skill {
        Skill::new("scroll_and_read", "Scroll and Read",
            "Scroll down the page and collect all visible text content.",
            SkillCategory::General)
            .save_steps(10)
            .with_parameters(vec![
                SkillParameter::new("max_scrolls", false).with_default("5").with_description("Maximum number of scrolls")
            ])
            .with_triggers(vec!["read the page", "scroll and read", "read all content"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Read current screen"),
                SkillStep::new("swipe").with_params(vec![("direction", "up")])
                    .with_description("Scroll down"),
                SkillStep::new("get_screen_info").with_description("Read after scroll"),
                SkillStep::new("swipe").with_params(vec![("direction", "up")])
                    .with_description("Scroll down more"),
                SkillStep::new("get_screen_info").with_description("Read after scroll"),
            ])
            .with_fallback("Scroll down the page and collect all visible text.")
    }

    pub fn copy_screen_text() -> Skill {
        Skill::new("copy_screen_text", "Copy Screen Text",
            "Extract all visible text from the current screen.",
            SkillCategory::General)
            .save_steps(3)
            .with_triggers(vec!["copy text", "extract text", "read screen"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Read screen content"),
            ])
            .with_fallback("Read and return all visible text on the screen.")
    }

    pub fn accept_permission() -> Skill {
        Skill::new("accept_permission", "Accept Permission",
            "Accept permission dialogs, terms, or consent popups. Use when a dialog asks to Allow, Accept, or Agree.",
            SkillCategory::Dismiss)
            .save_steps(3)
            .with_triggers(vec!["accept permission", "allow permission", "grant access"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Check for permission dialog"),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Allow")])
                    .with_description("Tap Allow").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "While using the app")])
                    .with_description("Tap While using").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Accept")])
                    .with_description("Tap Accept").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Agree")])
                    .with_description("Tap Agree").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "Continue")])
                    .with_description("Tap Continue").optional(),
                SkillStep::new("find_and_tap").with_params(vec![("text", "I agree")])
                    .with_description("Tap I agree").optional(),
            ])
            .with_fallback("A permission or consent dialog appeared. Find and tap the Allow/Accept/Agree button.")
    }

    pub fn swipe_gesture() -> Skill {
        Skill::new("swipe_gesture", "Swipe Screen",
            "Swipe the screen in a direction. Use for Tinder swipes, Instagram stories, carousels, or any swipeable content.",
            SkillCategory::Navigation)
            .save_steps(2)
            .with_parameters(vec![
                SkillParameter::new("direction", true).with_description("Direction: left, right, up, or down")
            ])
            .with_steps(vec![
                SkillStep::new("swipe").with_params(vec![("direction", "{direction}")])
                    .with_description("Swipe {direction}"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "1000")])
                    .with_description("Wait for animation"),
            ])
            .with_fallback("Swipe the screen {direction}.")
    }

    pub fn go_back() -> Skill {
        Skill::new("go_back", "Go Back",
            "Press the back button to return to the previous screen.",
            SkillCategory::Navigation)
            .save_steps(1)
            .with_triggers(vec!["go back", "press back", "navigate back"])
            .with_steps(vec![
                SkillStep::new("system_key").with_params(vec![("key", "back")])
                    .with_description("Press back"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "1000")])
                    .with_description("Wait for transition"),
            ])
            .with_fallback("Press the back button.")
    }

    pub fn wait_for_content() -> Skill {
        Skill::new("wait_for_content", "Wait for Content",
            "Wait for new content to appear on screen. Polls up to 15 seconds.",
            SkillCategory::General)
            .save_steps(5)
            .with_triggers(vec!["wait for content", "wait for loading", "wait for response"])
            .with_steps(vec![
                SkillStep::new("get_screen_info").with_description("Capture initial screen state"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "3000")])
                    .with_description("Wait 3s"),
                SkillStep::new("get_screen_info").with_description("Check for new content"),
                SkillStep::new("wait").with_params(vec![("duration_ms", "3000")])
                    .with_description("Wait 3s more"),
                SkillStep::new("get_screen_info").with_description("Final check"),
            ])
            .with_fallback("Wait for new content to appear on the screen. Check every 3 seconds.")
    }

    pub fn all() -> Vec<Skill> {
        vec![
            Self::search_in_app(),
            Self::submit_form(),
            Self::dismiss_popup(),
            Self::scroll_and_read(),
            Self::copy_screen_text(),
            Self::accept_permission(),
            Self::swipe_gesture(),
            Self::go_back(),
            Self::wait_for_content(),
        ]
    }
}
