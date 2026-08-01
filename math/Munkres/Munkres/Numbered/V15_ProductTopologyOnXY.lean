import Munkres.Closure.Subtype
import Munkres.Mathlib.AccPt.Basic
import Munkres.Mathlib.Disjoint
import Munkres.Subtype.Topology

open Set Topology Filter TopologicalSpace

universe u v

variable {α : Type u} {β : Type v} [TopologicalSpace α] [TopologicalSpace β]

-- Mathlib's Product Topology.
@[reducible]
private def tₚ : TopologicalSpace (α × β) := instTopologicalSpaceProd (X := α) (Y := β)

private def B := { p | ∃ (U : Set α) (V : Set β), IsOpen U ∧ IsOpen V ∧ p = U ×ˢ V }

-- private lemma l₀ {B : Set (Set α)} :

example : @IsTopologicalBasis (α × β) tₚ B := by
  set B := B (α := α) (β := β)
  have exists_subset_inter : ∀ t₁ ∈ B, ∀ t₂ ∈ B, ∀ x ∈ t₁ ∩ t₂, ∃ t₃ ∈ B, x ∈ t₃ ∧ t₃ ⊆ t₁ ∩ t₂
    := by --
    intro t₁ ⟨u₁, v₁, hu₁, hv₁, ht₁⟩ t₂ ⟨u₂, v₂, hu₂, hv₂, ht₂⟩ z ⟨hz₁, hz₂⟩
    use (u₁ ∩ u₂) ×ˢ (v₁ ∩ v₂)
    refine ⟨?_, ?_, ?_⟩
    · exact ⟨u₁ ∩ u₂, v₁ ∩ v₂, hu₁.inter hu₂, hv₁.inter hv₂, rfl⟩
    · rw [ht₁] at hz₁
      rw [ht₂] at hz₂
      exact mk_mem_prod ⟨hz₁.1, hz₂.1⟩ ⟨hz₁.2, hz₂.2⟩
    · rw [prod_subset_iff]
      intro x hx y hy
      refine ⟨?_, ?_⟩
      · rw [ht₁]
        exact mk_mem_prod hx.1 hy.1
      · rw [ht₂]
        exact mk_mem_prod hx.2 hy.2 -- ∎
  have sUnion_eq : ⋃₀ B = univ
    := by --
    refine eq_univ_of_univ_subset ?_
    have : univ ∈ B := by
      rw [<-univ_prod_univ]
      exact ⟨univ, univ, isOpen_univ, isOpen_univ, rfl⟩
    exact subset_sUnion_of_subset B univ (univ_subset_iff.mpr rfl) this -- ∎
  have hCover : ∀ x, ∃ b ∈ B, x ∈ b := by
    rw [<-Set.sUnion_eq_univ_iff]
    exact sUnion_eq
  refine { exists_subset_inter, sUnion_eq, eq_generateFrom := ?_ }
  · refine TopologicalSpace.ext ?_
    ext u : 2
    refine ⟨?_, ?_⟩
    · intro h
      sorry
    · sorry

-- example {U : Set α} {V : Set β} (hU : IsOpen U) (hV : IsOpen V)
--   :  := by
--   sorry

section S₀
end S₀
