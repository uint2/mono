import Mathlib.Data.List.TFAE
import Mathlib.Order.ConditionallyCompleteLattice.Basic
import Munkres.Mathlib.Set.As
import Mathlib.Tactic.TFAE

universe u v

variable {α : Type u} {ι : Type v}

/-- Everything inferrable from disjoint. Proof is overloaded with examples so
that the user can simplify. -/
theorem Disjoint.tfae {s t : Set α} : List.TFAE [ s ∩ t = ∅, Disjoint s t, s ⊆ tᶜ, t ⊆ sᶜ ]
  := by --
  tfae_have 2 → 1 := Disjoint.inter_eq
  tfae_have 2 ↔ 1 := Set.disjoint_iff_inter_eq_empty
  tfae_have 3 ↔ 2 := Set.subset_compl_iff_disjoint_right
  tfae_have 4 ↔ 2 := Set.subset_compl_iff_disjoint_left
  tfae_finish -- ∎

theorem Disjoint.as {X Y : Set α} (h : Disjoint X Y) (β : Set α) : Disjoint (X.as β) (Y.as β)
  := by --
  rw [Set.disjoint_iff_forall_ne]
  intro a (ha : ↑a ∈ X) b (hb : ↑b ∈ Y)
  contrapose! h
  refine Set.Nonempty.not_disjoint ⟨a, ha, ?_⟩
  subst h
  exact hb -- ∎

-- If two sets of subtypes are disjoint, then they are disjoint in any other
-- subtype too.
theorem Disjoint.subtype {Y : Set α} (X : Set α) {A B : Set Y} (h : Disjoint A B)
  : Disjoint ((A : Set α).as X) ((B : Set α).as X)
  := by --
  rw [<-Set.disjoint_image_iff Subtype.val_injective] at h
  exact h.as X -- ∎

theorem Set.disjoint_image_subtype_iff {X : Set α} {A B : Set X} :
  Disjoint (Subtype.val '' A) (Subtype.val '' B) ↔ Disjoint A B
  := by --
  exact Set.disjoint_image_iff Subtype.val_injective -- ∎

theorem Disjoint.image_subtype_iff {X : Set α} {A B : Set X} :
  Disjoint (Subtype.val '' A) (Subtype.val '' B) ↔ Disjoint A B
  := by --
  exact Set.disjoint_image_iff Subtype.val_injective -- ∎

lemma disjoint_assoc₂ {A B C : Set α} : Disjoint (A ∩ B) C ↔ Disjoint A (B ∩ C) := disjoint_assoc

theorem Disjoint.union_eq_univ_left_compl
  {A B : Set α} (h : Disjoint A B) :
  A ∪ B = Set.univ → Aᶜ = B
  := by --
  intro h_univ
  exact le_antisymm (Set.compl_subset_iff_union.mpr h_univ) (h.le_compl_left) -- ∎

theorem Disjoint.union_eq_univ_right_compl
  {A B : Set α} (h : Disjoint A B) :
  A ∪ B = Set.univ → Bᶜ = A
  := by --
  intro h_univ
  refine le_antisymm ?_ h.le_compl_right
  refine Set.compl_subset_iff_union.mpr ?_
  rw [Set.union_comm]
  exact h_univ -- ∎

theorem Disjoint.union_eq_univ_t₀
  {A B X : Set α} (h : Disjoint A B) :
  A ∪ B = Set.univ → X \ A = X ∩ B
  := by --
  intro union'
  refine le_antisymm ?_ ?_
  · intro x ⟨hxY, hxA⟩
    refine ⟨hxY, ?_⟩
    by_contra hxB
    have : x ∉ A ∪ B := by
      have h : x ∉ A ∧ x ∉ B := ⟨hxA, hxB⟩
      contrapose! h;
      exact h.resolve_left
    rw [union'] at this
    exact this trivial
  · intro x ⟨hxY, hxB⟩
    refine ⟨hxY, ?_⟩
    by_contra hxA
    have : (A ∩ B).Nonempty := ⟨x, hxA, hxB⟩
    refine this.ne_empty h.inter_eq -- ∎
