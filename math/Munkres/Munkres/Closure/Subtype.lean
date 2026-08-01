import Mathlib.Topology.Constructions
import Munkres.Mathlib.Set.As

import Munkres.Mathlib.Prelude

open Set

universe u

variable {α : Type u} [TopologicalSpace α] {Y A : Set α}

example (A : Set Y) {x : Y} : x ∈ closure A ↔ ↑x ∈ closure (Subtype.val '' A) := closure_subtype

--* Theorem 17.4: the closure of A in Y equals (closure A) ∩ Y.
theorem closure_subtype₂ (A : Set Y) : closure A = closure (A : Set α) ∩ Y
  := by --
  refine le_antisymm ?_ ?_
  · intro x hx
    rw [Subtype.coe_image] at hx
    obtain ⟨hxY, hx⟩ := hx
    exact ⟨closure_subtype.mp hx, hxY⟩
  · intro x ⟨hx, hxY⟩
    exact ⟨⟨x, hxY⟩, closure_subtype.mpr hx, rfl⟩ -- ∎

--* Theorem 17.4: If A ⊆ Y, the closure of A in Y equals (closure A) ∩ Y.
theorem closure_subtype₃ (hAY : A ⊆ Y) : closure (A.as Y) = closure A ∩ Y
  := by --
  rw [closure_subtype₂ (A.as Y)]
  refine congrArg (closure · ∩ Y) ?_
  exact Set.as_subtype_subset_eq hAY -- ∎

-- This counter example shows the necessity of A ⊆ Y above.
example : ∃ Y A : Set ℝ, ¬A ⊆ Y ∧ closure (A.as Y) ≠ closure A ∩ Y
  := by --
  let Y : Set ℝ := Icc 0 1
  let A : Set ℝ := Ioo 1 2
  use Y, A
  have hYA : Disjoint Y A := by
    refine disjoint_left.mpr ?_
    intro x ⟨hx₀, hx₁⟩
    exact notMem_Ioo_of_le hx₁
  have : A.as Y = ∅ := by
    dsimp only [Set.as]
    by_contra! h
    obtain ⟨x, hxA : x.val ∈ A⟩ := h
    have : (Y ∩ A).Nonempty := ⟨x.val, x.prop, hxA⟩
    rw [Set.nonempty_iff_ne_empty] at this
    exact this hYA.inter_eq
  rw [this, closure_empty, image_empty]
  rw [<-Set.nonempty_iff_empty_ne]
  refine ⟨?_, 1, ?_, ?_⟩
  · rw [not_subset]
    obtain ⟨d, hd₁, hd₂⟩ := exists_between (one_lt_two (α := ℝ))
    exact ⟨d, ⟨hd₁, hd₂⟩, notMem_Icc_of_gt hd₁⟩
  · rw [closure_Ioo (OfNat.one_ne_ofNat 2)]
    exact ⟨le_rfl, one_le_two⟩
  · exact ⟨zero_le_one, le_rfl⟩ -- ∎
