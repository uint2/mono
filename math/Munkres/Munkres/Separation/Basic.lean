import Mathlib.Topology.Basic

import Munkres.Defs.Separation
import Munkres.Mathlib.Disjoint

universe u

variable {α : Type u} [TopologicalSpace α] {U V : Set α}


section IsSeparation
variable (h : IsSeparation U V)
include h

lemma IsSeparation.isOpen_left : IsOpen U
  := by --
  exact h.left'.1 -- ∎
lemma IsSeparation.isOpen_right : IsOpen V
  := by --
  exact h.right'.1 -- ∎
lemma IsSeparation.left_compl : Uᶜ = V
  := by --
  exact h.disjoint'.union_eq_univ_left_compl h.union' -- ∎
lemma IsSeparation.right_compl : Vᶜ = U
  := by --
  exact h.disjoint'.union_eq_univ_right_compl h.union' -- ∎
lemma IsSeparation.isClosed_left : IsClosed U
  := by --
  rw [<-isOpen_compl_iff, h.left_compl]
  exact h.right'.1 -- ∎
lemma IsSeparation.isClosed_right : IsClosed V
  := by --
  rw [<-isOpen_compl_iff, h.right_compl]
  exact h.left'.1 -- ∎
lemma IsSeparation.isClopen_left : IsClopen U
  := by --
  exact ⟨h.isClosed_left, h.isOpen_left⟩ -- ∎
lemma IsSeparation.isClopen_right : IsClopen V
  := by --
  exact ⟨h.isClosed_right, h.isOpen_right⟩ -- ∎
lemma IsSeparation.ne_univ_left : U ≠ Set.univ
  := by --
  rw [<-h.right_compl]
  exact Set.compl_ne_univ.mpr h.right'.2 -- ∎
lemma IsSeparation.ne_univ_right : V ≠ Set.univ
  := by --
  rw [<-h.left_compl]
  exact Set.compl_ne_univ.mpr h.left'.2 -- ∎
lemma IsSeparation.symm : IsSeparation V U
  := by --
  exact {
    left' := h.right', right' := h.left'
    disjoint' := by rw [disjoint_comm]; exact h.disjoint'
    union' := by rw [Set.union_comm]; exact h.union' } -- ∎
omit h in
lemma IsSeparation.comm : IsSeparation U V ↔ IsSeparation V U
  := by --
  exact ⟨(·.symm), (·.symm)⟩ -- ∎
lemma IsSeparation.sdiff_eq_inter (X : Set α) : X \ U = X ∩ V
  := by --
  exact h.disjoint'.union_eq_univ_t₀ h.union' -- ∎
lemma IsSeparation.inter_eq_sdiff (X : Set α) : X ∩ U = X \ V
  := by --
  exact (h.symm.sdiff_eq_inter X).symm -- ∎

end IsSeparation
