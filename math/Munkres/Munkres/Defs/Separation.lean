import Mathlib.Data.Set.BooleanAlgebra
import Mathlib.Order.CompletePartialOrder
import Mathlib.Topology.Defs.Basic

universe u

variable {α : Type u} [TopologicalSpace α]

/-- The existence of a separation implies that a space is not connected. -/
structure IsSeparation (U V : Set α) : Prop where
  left' : IsOpen U ∧ U.Nonempty
  right' : IsOpen V ∧ V.Nonempty
  disjoint' : Disjoint U V
  union' : U ∪ V = Set.univ

def HasSeparation (α : Type u) [TopologicalSpace α] : Prop := ∃ U V : Set α, IsSeparation U V
