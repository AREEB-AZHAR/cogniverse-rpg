use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitalType {
    Hunger,
    Thirst,
    Energy,
    Warmth,
    SocialNeed,
    Piety,
    Health,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityFilter {
    FoodSource,
    WaterSource,
    Tree,
    StoneRock,
    IronVein,
    Altar,
    PotentialMate,
    FriendlyNPC,
    HostilePredator,
    Campfire,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    GreaterThan,
    LessThan,
    Equal,
    And,
    Or,
    Add,
    Subtract,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuromodType {
    Dopamine,
    Serotonin,
    Noradrenaline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionChoice {
    IdleRest,
    MoveToNearest(EntityFilter),
    ForageFood,
    DrinkWater,
    ChopWood,
    MineStone,
    MineIron,
    WarmByFire,
    CraftTool,
    BuildShelter,
    CourtNearestMate,
    MakeAltarOffering,
    CastMagicRune,
    FleeThreat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Expr {
    Vital(VitalType),
    Constant(f32),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    DistanceTo(EntityFilter),
    HasItemInInventory(String),
    Neuromod(NeuromodType),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Stmt {
    IfElse(Expr, Box<Stmt>, Box<Stmt>),
    PriorityBranch(Vec<(Expr, Stmt)>),
    Act(ActionChoice),
    Sequence(Vec<Stmt>),
}

pub struct AstVmContext<'a> {
    pub hunger: f32,
    pub thirst: f32,
    pub energy: f32,
    pub warmth: f32,
    pub social_need: f32,
    pub piety: f32,
    pub health: f32,
    pub dopamine: f32,
    pub serotonin: f32,
    pub noradrenaline: f32,
    pub nearest_distances: &'a std::collections::HashMap<EntityFilter, f32>,
    pub inventory_has: &'a dyn Fn(&str) -> bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BehavioralGenome {
    pub root: Stmt,
    pub generation: u32,
    pub mutations_count: u32,
    pub fitness_score: f32,
}

impl BehavioralGenome {
    pub fn new_default() -> Self {
        // Safe survival default AST:
        // If Hunger > 0.65 -> Forage or Move to Food
        // Else If Thirst > 0.65 -> Drink or Move to Water
        // Else If Energy < 0.25 -> Rest
        // Else -> Priority selector (Craft, Court, Make Offering)
        let root = Stmt::IfElse(
            Expr::BinaryOp(
                Box::new(Expr::Vital(VitalType::Hunger)),
                Op::GreaterThan,
                Box::new(Expr::Constant(0.65)),
            ),
            Box::new(Stmt::IfElse(
                Expr::BinaryOp(
                    Box::new(Expr::DistanceTo(EntityFilter::FoodSource)),
                    Op::LessThan,
                    Box::new(Expr::Constant(2.0)),
                ),
                Box::new(Stmt::Act(ActionChoice::ForageFood)),
                Box::new(Stmt::Act(ActionChoice::MoveToNearest(EntityFilter::FoodSource))),
            )),
            Box::new(Stmt::IfElse(
                Expr::BinaryOp(
                    Box::new(Expr::Vital(VitalType::Thirst)),
                    Op::GreaterThan,
                    Box::new(Expr::Constant(0.65)),
                ),
                Box::new(Stmt::IfElse(
                    Expr::BinaryOp(
                        Box::new(Expr::DistanceTo(EntityFilter::WaterSource)),
                        Op::LessThan,
                        Box::new(Expr::Constant(2.0)),
                    ),
                    Box::new(Stmt::Act(ActionChoice::DrinkWater)),
                    Box::new(Stmt::Act(ActionChoice::MoveToNearest(EntityFilter::WaterSource))),
                )),
                Box::new(Stmt::IfElse(
                    Expr::BinaryOp(
                        Box::new(Expr::Vital(VitalType::Energy)),
                        Op::LessThan,
                        Box::new(Expr::Constant(0.25)),
                    ),
                    Box::new(Stmt::Act(ActionChoice::IdleRest)),
                    Box::new(Stmt::PriorityBranch(vec![
                        (
                            Expr::BinaryOp(
                                Box::new(Expr::Vital(VitalType::Piety)),
                                Op::GreaterThan,
                                Box::new(Expr::Constant(0.55)),
                            ),
                            Stmt::IfElse(
                                Expr::BinaryOp(
                                    Box::new(Expr::DistanceTo(EntityFilter::Altar)),
                                    Op::LessThan,
                                    Box::new(Expr::Constant(2.5)),
                                ),
                                Box::new(Stmt::Act(ActionChoice::MakeAltarOffering)),
                                Box::new(Stmt::Act(ActionChoice::MoveToNearest(EntityFilter::Altar))),
                            ),
                        ),
                        (
                            Expr::BinaryOp(
                                Box::new(Expr::Vital(VitalType::SocialNeed)),
                                Op::GreaterThan,
                                Box::new(Expr::Constant(0.5)),
                            ),
                            Stmt::Act(ActionChoice::CourtNearestMate),
                        ),
                        (
                            Expr::HasItemInInventory("Wood".into()),
                            Stmt::IfElse(
                                Expr::HasItemInInventory("Stone".into()),
                                Box::new(Stmt::Act(ActionChoice::BuildShelter)),
                                Box::new(Stmt::Act(ActionChoice::MoveToNearest(EntityFilter::StoneRock))),
                            ),
                        ),
                        (
                            Expr::Constant(1.0),
                            Stmt::IfElse(
                                Expr::BinaryOp(
                                    Box::new(Expr::DistanceTo(EntityFilter::Tree)),
                                    Op::LessThan,
                                    Box::new(Expr::Constant(1.8)),
                                ),
                                Box::new(Stmt::Act(ActionChoice::ChopWood)),
                                Box::new(Stmt::Act(ActionChoice::MoveToNearest(EntityFilter::Tree))),
                            ),
                        ),
                    ])),
                )),
            )),
        );

        Self {
            root,
            generation: 1,
            mutations_count: 0,
            fitness_score: 10.0,
        }
    }

    /// Evaluates the AST with a 500-gas meter budget to prevent any infinite loops or lags
    pub fn evaluate(&self, ctx: &AstVmContext) -> (ActionChoice, u32) {
        let mut gas = 500;
        let mut chosen_action = None;
        Self::eval_stmt(&self.root, ctx, &mut gas, &mut chosen_action);
        let action = chosen_action.unwrap_or(ActionChoice::IdleRest);
        (action, 500 - gas)
    }

    fn eval_expr(expr: &Expr, ctx: &AstVmContext, gas: &mut u32) -> f32 {
        if *gas == 0 {
            return 0.0;
        }
        *gas -= 1;

        match expr {
            Expr::Vital(vital) => match vital {
                VitalType::Hunger => ctx.hunger,
                VitalType::Thirst => ctx.thirst,
                VitalType::Energy => ctx.energy,
                VitalType::Warmth => ctx.warmth,
                VitalType::SocialNeed => ctx.social_need,
                VitalType::Piety => ctx.piety,
                VitalType::Health => ctx.health,
            },
            Expr::Constant(val) => *val,
            Expr::BinaryOp(left, op, right) => {
                let l_val = Self::eval_expr(left, ctx, gas);
                let r_val = Self::eval_expr(right, ctx, gas);
                match op {
                    Op::GreaterThan => if l_val > r_val { 1.0 } else { 0.0 },
                    Op::LessThan => if l_val < r_val { 1.0 } else { 0.0 },
                    Op::Equal => if (l_val - r_val).abs() < 0.001 { 1.0 } else { 0.0 },
                    Op::And => if l_val > 0.5 && r_val > 0.5 { 1.0 } else { 0.0 },
                    Op::Or => if l_val > 0.5 || r_val > 0.5 { 1.0 } else { 0.0 },
                    Op::Add => l_val + r_val,
                    Op::Subtract => l_val - r_val,
                }
            }
            Expr::DistanceTo(filter) => {
                *ctx.nearest_distances.get(filter).unwrap_or(&999.0)
            }
            Expr::HasItemInInventory(item_name) => {
                if (ctx.inventory_has)(item_name) { 1.0 } else { 0.0 }
            }
            Expr::Neuromod(nm) => match nm {
                NeuromodType::Dopamine => ctx.dopamine,
                NeuromodType::Serotonin => ctx.serotonin,
                NeuromodType::Noradrenaline => ctx.noradrenaline,
            },
        }
    }

    fn eval_stmt(stmt: &Stmt, ctx: &AstVmContext, gas: &mut u32, action_out: &mut Option<ActionChoice>) {
        if *gas == 0 || action_out.is_some() {
            return;
        }
        *gas -= 1;

        match stmt {
            Stmt::IfElse(cond, then_branch, else_branch) => {
                let cond_res = Self::eval_expr(cond, ctx, gas);
                if cond_res > 0.5 {
                    Self::eval_stmt(then_branch, ctx, gas, action_out);
                } else {
                    Self::eval_stmt(else_branch, ctx, gas, action_out);
                }
            }
            Stmt::PriorityBranch(branches) => {
                for (cond, branch_stmt) in branches {
                    if action_out.is_some() || *gas == 0 {
                        break;
                    }
                    let score = Self::eval_expr(cond, ctx, gas);
                    if score > 0.5 {
                        Self::eval_stmt(branch_stmt, ctx, gas, action_out);
                        break;
                    }
                }
            }
            Stmt::Act(action) => {
                *action_out = Some(*action);
            }
            Stmt::Sequence(stmts) => {
                for s in stmts {
                    if action_out.is_some() || *gas == 0 {
                        break;
                    }
                    Self::eval_stmt(s, ctx, gas, action_out);
                }
            }
        }
    }

    /// Mutates the AST using Darwin-Gödel principles:
    /// High noradrenaline (stress/novelty) scales the magnitude of mutation
    pub fn mutate(&mut self, noradrenaline_plasticity: f32) {
        let mut rng = rand::thread_rng();
        self.mutations_count += 1;
        self.generation += 1;

        let mutation_scale = 0.05 + 0.15 * noradrenaline_plasticity.clamp(0.0, 1.0);
        Self::mutate_stmt(&mut self.root, &mut rng, mutation_scale);
    }

    fn mutate_stmt<R: rand::Rng>(stmt: &mut Stmt, rng: &mut R, scale: f32) {
        match stmt {
            Stmt::IfElse(cond, then_b, else_b) => {
                Self::mutate_expr(cond, rng, scale);
                if rng.gen_bool(0.5) {
                    Self::mutate_stmt(then_b, rng, scale);
                } else {
                    Self::mutate_stmt(else_b, rng, scale);
                }
            }
            Stmt::PriorityBranch(branches) => {
                for (expr, s) in branches.iter_mut() {
                    if rng.gen_bool(0.3) {
                        Self::mutate_expr(expr, rng, scale);
                        Self::mutate_stmt(s, rng, scale);
                    }
                }
            }
            Stmt::Act(action) => {
                if rng.gen_bool(0.15) {
                    // Small chance to mutate action choice
                    let choices = [
                        ActionChoice::IdleRest,
                        ActionChoice::ForageFood,
                        ActionChoice::DrinkWater,
                        ActionChoice::ChopWood,
                        ActionChoice::MineStone,
                        ActionChoice::CraftTool,
                        ActionChoice::CourtNearestMate,
                        ActionChoice::MakeAltarOffering,
                    ];
                    *action = choices[rng.gen_range(0..choices.len())];
                }
            }
            Stmt::Sequence(stmts) => {
                for s in stmts.iter_mut() {
                    Self::mutate_stmt(s, rng, scale);
                }
            }
        }
    }

    fn mutate_expr<R: rand::Rng>(expr: &mut Expr, rng: &mut R, scale: f32) {
        match expr {
            Expr::Constant(val) => {
                let delta = rng.gen_range(-scale..scale);
                *val = (*val + delta).clamp(0.05, 0.95);
            }
            Expr::BinaryOp(left, _, right) => {
                if rng.gen_bool(0.5) {
                    Self::mutate_expr(left, rng, scale);
                } else {
                    Self::mutate_expr(right, rng, scale);
                }
            }
            _ => {}
        }
    }

    /// Mendelian Crossover between two parent genomes
    pub fn crossover(parent_a: &Self, parent_b: &Self) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let new_root = match (&parent_a.root, &parent_b.root) {
            (Stmt::IfElse(cond_a, then_a, _), Stmt::IfElse(_, _, else_b)) => {
                Stmt::IfElse(cond_a.clone(), then_a.clone(), else_b.clone())
            }
            _ => {
                if rng.gen_bool(0.5) {
                    parent_a.root.clone()
                } else {
                    parent_b.root.clone()
                }
            }
        };

        Self {
            root: new_root,
            generation: parent_a.generation.max(parent_b.generation) + 1,
            mutations_count: (parent_a.mutations_count + parent_b.mutations_count) / 2,
            fitness_score: (parent_a.fitness_score + parent_b.fitness_score) / 2.0,
        }
    }
}
