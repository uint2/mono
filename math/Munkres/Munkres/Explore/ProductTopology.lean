import Munkres.Mathlib.Prelude
import Munkres.Defs

open Set Munkres TopologicalSpace Filter Topology

universe u v

variable {Λ : Type v}

section One
-- We make |Λ| copies of the topological space X.
variable {X : Type u} [TopologicalSpace X]
  -- An arbitrary point.
  {x : Λ → X}
  -- An arbitrary index set.
  {J : Set Λ}
  -- A set of sequences. Very easy to check mem.
  {U : Set (Λ → X)}
  -- A sequence of sets. This is usually how we see sets in the wild.
  {V : Λ → Set X}

alias 𝓢 := SetLike.coe

example : TopologicalSpace (Π _ : Λ, X) := Pi.topologicalSpace
example : TopologicalSpace (Λ → X) := Pi.topologicalSpace
example : (Π _ : Λ, X) = (Λ → X) := rfl

-- Convert V to something that can easily check mem.
example : Set (Λ → X) := univ.pi V

example : x ∈ univ.pi V ↔ ∀ α, x α ∈ V α := mem_univ_pi
example {J : Set Λ} : x ∈ J.pi V ↔ ∀ α ∈ J, x α ∈ V α := mem_pi

private lemma isOpen_pi_iff₂ : IsOpen U ↔
  ∀ x ∈ U, ∃ (J : Finset Λ), ∃ u, (∀ α ∈ J, u α ∈ nhds' (x α)) ∧ (𝓢 J).pi u ⊆ U
  := by --
  rw [isOpen_pi_iff]
  simp only [nhds'.mem_iff]
  exact Multiplicative.forall -- ∎

example (hJ : J.Finite) : (∀ α ∈ J, IsOpen (V α)) → IsOpen (J.pi V) := isOpen_set_pi hJ
-- Probably false.
example (hJ : J.Finite) : (∀ α ∈ J, IsOpen (V α)) ↔ IsOpen (J.pi V)
  := by --
  refine ⟨isOpen_set_pi hJ, ?_⟩
  intro h α hα
  rw [isOpen_pi_iff] at h
  sorry -- ∎

-- Open sets in the product topology
example : IsOpen U ↔
  ∀ x ∈ U, ∃ (J : Finset Λ), ∃ u, (∀ α ∈ J, IsOpen (u α) ∧ x α ∈ u α) ∧ (𝓢 J).pi u ⊆ U
  := by --
  exact isOpen_pi_iff -- ∎

-- Probably false.
example : IsOpen U ↔ ∀ x ∈ U, ∃ (J : Finset Λ), ∃ u,
  (IsOpen ((𝓢 J).pi u)) ∧ (∀ α ∈ J, x α ∈ u α) ∧ (𝓢 J).pi u ⊆ U
  := by --
  rw [isOpen_pi_iff]
  refine forall₂_congr ?_
  intro x hxU
  constructor
  · intro ⟨J, u, h₁, h₂⟩
    refine ⟨J, u, ?_, ?_, h₂⟩
    · exact isOpen_set_pi J.finite_toSet fun α hα ↦ (h₁ α hα).1
    · exact fun α hα ↦ (h₁ α hα).2
  · intro ⟨J, u, h₁, h₂, h₃⟩
    refine ⟨J, u, ?_, h₃⟩
    intro α hα
    refine ⟨?_, h₂ α hα⟩
    clear h₂ h₃
    sorry -- ∎

-- Basis structure.
private def B : Set (Set (Λ → X)) := { p | ∃ (J : Finset Λ) (u : Λ → Set X),
  (∀ α ∈ J, IsOpen (u α)) ∧ p = (𝓢 J).pi u }

private lemma hB : IsTopologicalBasis (α := Λ → X) B
  := by --
  refine isTopologicalBasis_of_isOpen_of_nhds ?_ ?_
  · dsimp only [B]
    intro b ⟨J, u, hu, heq⟩
    subst heq
    exact isOpen_set_pi J.finite_toSet hu
  · intro x U hxU hU
    rw [isOpen_pi_iff] at hU
    specialize hU x hxU
    obtain ⟨J, u, hu, hU⟩ := hU
    let v : Set (Λ → X) := (SetLike.coe J).pi u
    refine ⟨v, ?_, ?_, hU⟩
    · refine ⟨J, u, ?_, rfl⟩
      intro α hαJ
      exact (hu α hαJ).1
    · rw [Set.mem_pi]
      intro α hαJ
      exact (hu α hαJ).2 -- ∎

example : IsOpen U ↔ ∀ x ∈ U, ∃ b ∈ B, x ∈ b ∧ b ⊆ U := by
  have hB : IsTopologicalBasis B := hB (X := X) (Λ := Λ)
  exact hB.isOpen_iff

example {𝓤 : Set (Λ → X)} : List.TFAE [
  IsOpen 𝓤,
  ∀ x ∈ 𝓤, ∃ b ∈ B, x ∈ b ∧ b ⊆ 𝓤,
  ∀ x ∈ 𝓤, ∃ (J : Finset Λ), ∃ U, (∀ α ∈ J, IsOpen (U α) ∧ x α ∈ U α) ∧ (𝓢 J).pi U ⊆ 𝓤,
  ]
  := by --
  tfae_have 1 ↔ 2 := hB.isOpen_iff
  tfae_have 2 ↔ 3 := by
    refine forall₂_congr ?_
    intro x hxU
    constructor
    · intro ⟨b, ⟨Λ', u, hu, heq⟩, hxb, hbU⟩
      subst heq
      use Λ', u
      refine ⟨ ?_, hbU⟩
      intro α hα
      exact ⟨hu α hα, hxb α hα⟩
    · intro ⟨Λ', u, hu, hbU⟩
      use (𝓢 Λ').pi u
      refine ⟨?_, ?_, hbU⟩
      · use Λ', u
        refine ⟨?_, rfl⟩
        intro α hα
        exact (hu α hα).1
      · intro α hα
        exact (hu α hα).2
  tfae_finish -- ∎

-- Λ'.pi U is the set (Π α ∈ Λ', U α) × (Π α ∉ Λ', univ).
example (U : Λ → Set X) (Λ' : Set Λ) : univ.pi U ⊆ Λ'.pi U :=
  pi_mono' (fun _ _ ↦ le_rfl) (subset_univ _)

end One

section Many
-- Each X(α) is its own topological space. So we have arbitrarily many
-- topological spaces.

variable {X : Λ → Type u} [∀ α, TopologicalSpace (X α)]
  {x : Π α, X α}
  {U : Set (Π α, X α)}
  {V : Π α, Set (X α)}

example : TopologicalSpace (Π α, X α) := Pi.topologicalSpace

end Many
