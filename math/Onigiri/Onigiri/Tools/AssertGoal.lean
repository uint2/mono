/-
Copyright (c) 2026 Nguyen Vu Khang. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Nguyen Vu Khang
-/
module

public import Lean.Elab.Tactic.Basic

import Mathlib.Tactic.ToAdditive

/-! Checkpoint for proofs, to assert that the goal is definitionally equal to
  the suggested goal, without changing anything. -/

section AssertGoal
open Lean Elab Tactic Meta

/--
Checks that the current goal is definitionally equal to the provided goal. No-op
by default, and errors out if the goals do not match.
-/
elab "assert_goal " t:term : tactic => withMainContext do
  let goal ← getMainGoal
  let target ← goal.getType
  let expected ← Term.elabTermEnsuringType t (← inferType target)
  if ← isDefEq target expected then
    pure () -- No-op when definitionally equal
  else
    Lean.logError m!"goal not definitionally equal to {expected}"
    pure ()

end AssertGoal

-- Allow this tactic to be no-op without warning.
allow_unused_tactic tacticAssert_goal_
