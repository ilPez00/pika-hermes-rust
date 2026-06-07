use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub id: String,
    pub name: String,
    pub body: String,
    pub keywords: Vec<String>,
}

pub struct PlaybookManager {
    playbooks: Vec<Playbook>,
}

impl PlaybookManager {
    pub fn new() -> Self {
        Self {
            playbooks: Self::default_playbooks(),
        }
    }

    pub fn match_playbook(&self, query: &str) -> Option<&Playbook> {
        let q = query.to_lowercase();
        let mut best_match: Option<&Playbook> = None;
        let mut best_score = 0;

        for pb in &self.playbooks {
            let score = pb.keywords.iter().filter(|k| q.contains(&k.to_lowercase())).count();
            if score > best_score {
                best_score = score;
                best_match = Some(pb);
            }
        }

        best_match
    }

    fn default_playbooks() -> Vec<Playbook> {
        vec![
            Playbook {
                id: "send_message".into(),
                name: "Send a Message".into(),
                body: "1. Open the messaging app\n2. Find the contact\n3. Type the message\n4. Send it".into(),
                keywords: vec!["send".into(), "message".into(), "text".into(), "whatsapp".into()],
            },
            Playbook {
                id: "make_call".into(),
                name: "Make a Phone Call".into(),
                body: "1. Open the phone app\n2. Find the contact\n3. Tap the call button".into(),
                keywords: vec!["call".into(), "phone".into(), "dial".into()],
            },
            Playbook {
                id: "check_battery".into(),
                name: "Check Battery".into(),
                body: "Use get_device_info(category='battery') to check battery".into(),
                keywords: vec!["battery".into(), "charge".into(), "power".into()],
            },
        ]
    }
}
