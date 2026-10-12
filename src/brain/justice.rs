use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeType {
    TheftOfFood,
    DesecrationOfAltar,
    PhysicalAssault,
    HaremInfidelity,
    HoardingDuringFamine,
}

impl CrimeType {
    pub fn severity(&self) -> f32 {
        match self {
            CrimeType::TheftOfFood => 0.4,
            CrimeType::DesecrationOfAltar => 0.8,
            CrimeType::PhysicalAssault => 0.7,
            CrimeType::HaremInfidelity => 0.5,
            CrimeType::HoardingDuringFamine => 0.6,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrievanceRecord {
    pub offender_id: usize,
    pub victim_id: usize,
    pub crime: CrimeType,
    pub tick: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SocialJusticeLedger {
    pub reputations: std::collections::HashMap<usize, f32>, // -1.0 (Outlaw) to 1.0 (Honored Elder)
    pub grievances: Vec<GrievanceRecord>,
}

impl SocialJusticeLedger {
    pub fn new() -> Self {
        Self {
            reputations: std::collections::HashMap::new(),
            grievances: Vec::new(),
        }
    }

    pub fn get_reputation(&self, agent_id: usize) -> f32 {
        *self.reputations.get(&agent_id).unwrap_or(&0.5)
    }

    pub fn report_crime(&mut self, offender_id: usize, victim_id: usize, crime: CrimeType, tick: u64) {
        let severity = crime.severity();
        let rep = self.reputations.entry(offender_id).or_insert(0.5);
        *rep = (*rep - severity).clamp(-1.0, 1.0);

        self.grievances.push(GrievanceRecord {
            offender_id,
            victim_id,
            crime,
            tick,
        });

        if self.grievances.len() > 50 {
            self.grievances.remove(0);
        }
    }

    pub fn reward_good_deed(&mut self, agent_id: usize, bonus: f32) {
        let rep = self.reputations.entry(agent_id).or_insert(0.5);
        *rep = (*rep + bonus).clamp(-1.0, 1.0);
    }
}
