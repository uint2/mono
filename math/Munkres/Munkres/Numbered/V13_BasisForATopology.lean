import Munkres.Mathlib.Prelude
import Munkres.Mathlib.IsOpen

open Set TopologicalSpace Topology

universe u

variable {α : Type u} {B : Set (Set α)}

-- The reason why we reduce all finite intersection arguments to just the
-- intersection of two elements — that we can extend it to finite intersections
-- by induction.
example
  (h : ∀ b₁ ∈ B, ∀ b₂ ∈ B, b₁ ∩ b₂ ∈ B)
  (h_univ : univ ∈ B)
  : ∀ s ⊆ B, s.Finite → ⋂₀ s ∈ B
  := by --
  intro s hsB hsF
  induction s, hsF using Set.Finite.induction_on with
  | empty =>
    rw [sInter_empty]
    exact h_univ
  | @insert u s hus hsF ih =>
    have huB : u ∈ B := hsB <| mem_insert u s
    if hs₀ : s = ∅ then
      subst hs₀
      simp only [insert_empty_eq, sInter_singleton]
      exact huB
    else
    replace hs₀ : s.Nonempty := nonempty_iff_ne_empty.mpr hs₀
    specialize ih ((subset_insert u s).trans hsB)
    rw [sInter_insert u s]
    exact h u huB _ ih -- ∎

-- Same as the above, except that we swap out the requirement that univ ∈ B for
-- the constraint that we're talking only about non-empty intersections.
example
  (h : ∀ b₁ ∈ B, ∀ b₂ ∈ B, b₁ ∩ b₂ ∈ B)
  : ∀ s ⊆ B, s.Finite → s.Nonempty → ⋂₀ s ∈ B
  := by --
  intro s hsB hsF hs₀
  induction s, hsF using Set.Finite.induction_on with
  | empty =>
    rw [sInter_empty]
    exact False.elim (Set.not_nonempty_empty hs₀)
  | @insert u s hus hsF ih =>
    have huB : u ∈ B := hsB <| mem_insert u s
    if hs₀ : s = ∅ then
      subst hs₀
      simp only [insert_empty_eq, sInter_singleton]
      exact huB
    else
    replace hs₀ : s.Nonempty := nonempty_iff_ne_empty.mpr hs₀
    specialize ih ((subset_insert u s).trans hsB)
    rw [sInter_insert u s]
    exact h u huB _ (ih hs₀) -- ∎

section S₁
--* Lemma 13.1
variable [TopologicalSpace α]

-- In other words, 𝓣 = collection of all unions of elements of 𝓑.
example {U : Set α} (hB : IsTopologicalBasis B) : IsOpen U ↔ ∃ s ⊆ B, U = ⋃₀ s
  := by --
  refine ⟨?_, ?_⟩
  · intro hU
    rw [hB.isOpen_iff] at hU
    exact b3d183c U hU
  · intro ⟨s, hs, heq⟩
    subst heq
    refine isOpen_sUnion ?_
    intro u hu
    exact hB.isOpen (hs hu) -- ∎

end S₁

section S₂
--* Lemma 13.2
variable [TopologicalSpace α]

example {C : Set (Set α)}
  (h₁ : ∀ c ∈ C, IsOpen c)
  (h₂ : ∀ u, IsOpen u → ∀ x ∈ u, ∃ c ∈ C, x ∈ c ∧ c ⊆ u)
  : IsTopologicalBasis C
  := by --
  have h₃ : ∀ x, ∃ b ∈ C, x ∈ b := by
    intro x
    obtain ⟨c, hcC, hxc, _⟩ := h₂ Set.univ isOpen_univ x trivial
    exact ⟨c, hcC, hxc⟩
  have h₄ : ∀ s ∈ C, ∀ t ∈ C, ∀ x ∈ s ∩ t, ∃ c ∈ C, x ∈ c ∧ c ⊆ s ∩ t := by
    intro s hs t ht x hx
    exact h₂ (s ∩ t) ((h₁ s hs).inter (h₁ t ht)) x hx
  have sUnion_eq := Set.sUnion_eq_univ_iff.mpr h₃
  exact {
    sUnion_eq,
    exists_subset_inter := h₄,
    eq_generateFrom := by
      refine TopologicalSpace.ext ?_
      ext u : 2
      rw [IsOpen_generateFrom_iff u h₄ sUnion_eq]
      refine ⟨h₂ u, ?_⟩
      intro h
      obtain ⟨s, hs, heq⟩ := b3d183c u h
      subst heq
      exact isOpen_sUnion fun t ht ↦ h₁ t (hs ht)
  } -- ∎

end S₂

section S₃
--* Lemma 13.3
variable [T : TopologicalSpace α] [T' : TopologicalSpace α]
  {B B' : Set (Set α)}
  (hB : @IsTopologicalBasis _ T B)
  (hB' : @IsTopologicalBasis _ T' B')

-- Note that T' is finer than T here.
example : T' ≤ T ↔ ∀ x, ∀ b ∈ B, x ∈ b → ∃ b' ∈ B', x ∈ b' ∧ b' ⊆ b
  := by --
  constructor
  · intro hle x b hbB hxb
    have hb : @IsOpen α T b := hB.isOpen (t := T) hbB
    have hb' : @IsOpen α T' b := hle b hb
    rw [hB'.isOpen_iff] at hb'
    exact hb' x hxb
  · intro h u hu
    rw [hB.isOpen_iff (t := T)] at hu
    rw [hB'.isOpen_iff]
    intro x hx
    obtain ⟨b, hbB, hxb, hbu⟩ := hu x hx
    obtain ⟨b', hbB', hxb', hbb⟩ := h x b hbB hxb
    exact ⟨b', hbB', hxb', hbb.trans hbu⟩ -- ∎

end S₃
