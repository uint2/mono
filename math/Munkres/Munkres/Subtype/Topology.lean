import Mathlib.Order.CompletePartialOrder
import Mathlib.Topology.Compactness.Compact

import Munkres.Defs.Subtype
import Munkres.Subtype.Induced

open Set

universe u

variable {α : Type u}

section SimpleCases
variable {X A : Set α} [TopologicalSpace X]

-- Empty and univ lemmas.

lemma isOpenIn_empty : IsOpenIn ∅ X
  := by --
  exact { isOpen' := isOpen_const, subset' := empty_subset X } -- ∎
lemma isOpenIn_univ : IsOpenIn X X
  := by --
  refine { isOpen' := ?_, subset' := le_rfl }
  rw [Subtype.coe_preimage_self]
  exact isOpen_univ -- ∎

lemma isClosedIn_empty : IsClosedIn ∅ X
  := by --
  exact { isClosed' := isClosed_const, subset' := empty_subset X } -- ∎
lemma isClosedIn_univ : IsClosedIn X X
  := by --
  refine { isClosed' := ?_, subset' := le_rfl }
  rw [Subtype.coe_preimage_self]
  exact isClosed_univ -- ∎

lemma isCompactIn_empty : IsCompactIn ∅ X
  := by --
  refine { isCompact' := ?_, subset' := empty_subset X }
  rw [preimage_empty]
  exact isCompact_empty -- ∎

-- Complement lemmas.

lemma IsClosedIn.isOpen_compl (h : IsClosedIn A X) : IsOpenIn (X \ A) X
  := by --
  refine { isOpen' := ?_, subset' := diff_subset }
  rw [<-Subtype.compl]
  exact h.isClosed'.isOpen_compl -- ∎
lemma IsOpenIn.isClosed_compl (h : IsOpenIn A X) : IsClosedIn (X \ A) X
  := by --
  refine { isClosed' := ?_, subset' := diff_subset }
  rw [<-Subtype.compl]
  exact h.isOpen'.isClosed_compl -- ∎

lemma IsClosedIn.iff_compl : IsClosedIn A X ↔ IsOpenIn (X \ A) X ∧ A ⊆ X
  := by --
  refine ⟨fun h ↦ ⟨h.isOpen_compl, h.subset'⟩, ?_⟩
  intro ⟨h, subset'⟩
  rw [<-Set.diff_diff_cancel_left subset']
  exact h.isClosed_compl -- ∎
lemma IsOpenIn.iff_compl : IsOpenIn A X ↔ IsClosedIn (X \ A) X ∧ A ⊆ X
  := by --
  refine ⟨fun h ↦ ⟨h.isClosed_compl, h.subset'⟩, ?_⟩
  intro ⟨h, subset'⟩
  rw [<-Set.diff_diff_cancel_left subset']
  exact h.isOpen_compl -- ∎

end SimpleCases

example [TopologicalSpace α] {A X : Set α}
  : IsOpenIn A X → IsOpen X → IsOpen A
  := by
  intro hAX hX
  have hA := hAX.isOpen'
  rw [isOpen_induced_iff₂] at hA
  obtain ⟨U, hU, hA⟩ := hA
  rw [Subtype.image_preimage_val] at hA
  -- have : X ∩ A = A := inter_eq_self_of_subset_right hAX.subset'
  rw [inter_eq_self_of_subset_right hAX.subset'] at hA
  subst hA
  refine hX.inter hU

section IsOpenIn
variable {X Y A : Set α}

theorem IsOpenIn.trans [TopologicalSpace α]
  : IsOpenIn A X → IsOpen X → IsOpen A
  := by --
  intro hAX hX
  have hA := hAX.isOpen'
  rw [isOpen_induced_iff₂] at hA
  obtain ⟨U, hU, hA⟩ := hA
  rw [Subtype.image_preimage_val] at hA
  rw [inter_eq_self_of_subset_right hAX.subset'] at hA
  subst hA
  exact hX.inter hU -- ∎

theorem IsOpenIn.iff_inter [TopologicalSpace X]
  : IsOpenIn (A ∩ X) X ∧ A ⊆ X ↔ IsOpenIn A X
  := by --
  constructor
  · intro ⟨h, hAX⟩
    refine { isOpen' := ?_, subset' := hAX }
    replace h := h.isOpen'
    rw [preimage_inter, Subtype.coe_preimage_self, inter_univ] at h
    exact h
  · intro h
    refine ⟨?_, h.subset'⟩
    rw [inter_eq_left.mpr h.subset']
    exact h -- ∎

theorem IsOpen.isOpenIn [TopologicalSpace α] (hA : IsOpen A) (X : Set α)
  : IsOpenIn (A ∩ X) X
  := by --
  refine { isOpen' := ?_, subset' := inter_subset_right }
  rw [isOpen_induced_iff]
  exact ⟨A, hA, (Subtype.preimage_coe_inter_self X A).symm⟩ -- ∎

theorem IsOpenIn.iff [TopologicalSpace α]
  : IsOpenIn A Y ↔ ∃ U, IsOpen U ∧ A = U ∩ Y
  := by --
  constructor
  · intro h
    obtain ⟨U, hU, heq⟩ := isOpen_induced_iff.mpr h.isOpen'
    refine ⟨U, hU, ?_⟩
    rw [<-Set.inter_eq_self_of_subset_right h.subset']
    rw [inter_comm U Y]
    rw [<-Subtype.preimage_coe_eq_preimage_coe_iff, heq]
  · intro ⟨U, hU, heq⟩
    subst heq
    exact hU.isOpenIn Y -- ∎

end IsOpenIn
section IsClosedIn
variable {X Y A : Set α}

theorem IsClosedIn.trans [TopologicalSpace α]
  : IsClosedIn A X → IsClosed X → IsClosed A
  := by --
  intro hAX hX
  have hA := hAX.isClosed'
  rw [isClosed_induced_iff₂] at hA
  obtain ⟨U, hU, hA⟩ := hA
  rw [Subtype.image_preimage_val] at hA
  rw [inter_eq_self_of_subset_right hAX.subset'] at hA
  subst hA
  exact hX.inter hU -- ∎

theorem IsClosedIn.iff_inter [TopologicalSpace X]
  : IsClosedIn (A ∩ X) X ∧ A ⊆ X ↔ IsClosedIn A X
  := by --
  constructor
  · intro ⟨h, hAX⟩
    refine { isClosed' := ?_, subset' := hAX }
    replace h := h.isClosed'
    rw [preimage_inter, Subtype.coe_preimage_self, inter_univ] at h
    exact h
  · intro h
    refine ⟨?_, h.subset'⟩
    rw [inter_eq_left.mpr h.subset']
    exact h -- ∎

theorem IsClosed.isClosedIn [TopologicalSpace α] (hA : IsClosed A) (X : Set α)
  : IsClosedIn (A ∩ X) X
  := by --
  refine { isClosed' := ?_, subset' := inter_subset_right }
  rw [isClosed_induced_iff]
  exact ⟨A, hA, (Subtype.preimage_coe_inter_self X A).symm⟩ -- ∎

theorem IsClosedIn.iff [TopologicalSpace α]
  : IsClosedIn A Y ↔ ∃ U, IsClosed U ∧ A = U ∩ Y
  := by --
  constructor
  · intro h
    obtain ⟨U, hU, heq⟩ := isClosed_induced_iff.mp h.isClosed'
    refine ⟨U, hU, ?_⟩
    rw [<-Set.inter_eq_self_of_subset_right h.subset']
    rw [inter_comm U Y]
    rw [<-Subtype.preimage_coe_eq_preimage_coe_iff, heq]
  · intro ⟨U, hU, heq⟩
    subst heq
    exact hU.isClosedIn Y -- ∎

end IsClosedIn
