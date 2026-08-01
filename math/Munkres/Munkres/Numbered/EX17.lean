import Mathlib.Data.Set.Lattice
import Mathlib.Topology.Defs.Basic

open Set

universe u

variable {α : Type u}

section Q₁
-- Let 𝓒 be a collection of subsets of the set X. Suppose that ∅ and X are in 𝓒,
-- and that finite unions and arbitrary intersections of elements of 𝓒 are in 𝓒.
-- Show that the collection T = { X - C | C ∈ 𝓒 } is a topology on X.
@[reducible]
private def q₁ {C : Set (Set α)}
  (h_empty : ∅ ∈ C)
  (h_inter : ∀ s ⊆ C, ⋂₀ s ∈ C)
  (h_union : ∀ s ∈ C, ∀ t ∈ C, s ∪ t ∈ C)
  : TopologicalSpace α := by
  let T := C.image (·ᶜ)
  exact {
    IsOpen u := u ∈ T
    isOpen_univ := ⟨∅, h_empty, compl_empty⟩
    isOpen_inter s t hs ht := by
      have hs : sᶜ ∈ C := (mem_compl_image s C).mp hs
      have ht : tᶜ ∈ C := (mem_compl_image t C).mp ht
      have : sᶜ ∪ tᶜ ∈ C := h_union sᶜ hs tᶜ ht
      exact ⟨sᶜ ∪ tᶜ, this, (inter_eq_compl_compl_union_compl _ _).symm⟩
    isOpen_sUnion s hs := by
      let t := s.image (·ᶜ)
      have ht : t ⊆ C := by
        intro ζ ⟨u, hu, heq⟩
        subst heq
        exact (mem_compl_image u C).mp (hs u hu)
      refine ⟨⋂₀ t, h_inter t ht, ?_⟩
      dsimp only [t]
      simp only [sInter_image, compl_iInter, compl_compl]
      exact sUnion_eq_biUnion.symm
  }

end Q₁
