import Mathlib.Topology.Defs.Basic

namespace Munkres

universe u

variable {α : Type u} [TopologicalSpace α]

/-- Munkres' definition of neighborhood. In the sense that if U ∈ nhds' x, then
U is an open set that contains x. -/
def nhds' (x : α) := { u : Set α | IsOpen u ∧ x ∈ u }

theorem nhds'.mem_iff {U : Set α} {x : α} : U ∈ nhds' x ↔ IsOpen U ∧ x ∈ U
  := by --
  exact Eq.to_iff rfl -- ∎
theorem nhds'.mem_iff' {U : Set α} {x : α} : U ∈ nhds' x ↔ x ∈ U ∧ IsOpen U
  := by --
  rw [mem_iff]
  exact And.comm -- ∎

/-- A point `x` in a topological space is _isolated_ if `{x}` is open. -/
def IsIsolated (x : α) : Prop := IsOpen {x}

end Munkres
