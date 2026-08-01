import Munkres.Mathlib.Prelude

open Set TopologicalSpace

universe u

variable {α : Type u} [LinearOrder α]

/-
Too much trouble to handle all the cases, and for not much foreseeable use.

def OrderTopologyBasis := range Iio ∪ { Ioo a b | (a : α) (b : α) } ∪ range Ioi

private alias B := OrderTopologyBasis

private lemma OrderTopologyBasis.mem {b : Set α} :
  b ∈ B ↔ b ∈ range Iio ∨ b ∈ { Ioo a b | (a : α) (b : α) } ∨ b ∈ range Ioi
  := by
  simp only [<-or_assoc, mem_range]
  rfl

private lemma OrderTopologyBasis.mem_Iio {a : α} : Iio a ∈ B
  := by --
  exact Or.inl (Or.inl (mem_range_self a)) -- ∎
private lemma OrderTopologyBasis.mem_Ioi {a : α} : Ioi a ∈ B
  := by --
  exact Or.inr (mem_range_self a) -- ∎
private lemma OrderTopologyBasis.mem_Ioo {a b : α} : Ioo a b ∈ B
  := by --
  refine Or.inl (Or.inr ⟨a, b, rfl⟩) -- ∎

theorem OrderTopologyBasis.b₁ : ∀ x : α, ∃ b ∈ B, x ∈ b := by
  intro x
  if hlt : ∃ y, y < x then
    obtain ⟨y, hy⟩ := hlt
    exact ⟨Ioi y, mem_Ioi, hy⟩
  else
  if hgt : ∃ y, y > x then
    obtain ⟨y, hy⟩ := hgt
    exact ⟨Iio y, mem_Iio, hy⟩
  else
  push_neg at hlt hgt
  use Ioo x x
  sorry
  -- refine ⟨?_, le_rfl, le_rfl⟩
  -- exact Or.inl (Or.inr ⟨x, x, rfl⟩)

theorem OrderTopologyBasis.b₂
  : ∀ b₁ ∈ @B α _, ∀ b₂ ∈ B, ∀ x ∈ b₁ ∩ b₂, ∃ b ∈ B, x ∈ b ∧ b ⊆ b₁ ∩ b₂
  := by
  intro b₁ hb₁ b₂ hb₂ x hx
  rw [mem] at hb₁ hb₂
  rcases hb₁ with h₁_Iio | h₁_Ioo | h₁_Ioi
  · rcases hb₂ with h₂_Iio | h₂_Ioo | h₂_Ioi
    · obtain ⟨a, heq⟩ := h₁_Iio; subst heq
      obtain ⟨b, heq⟩ := h₂_Iio; subst heq
      let c := min a b
      have : Iio a ∩ Iio b = Iio c := Iio_inter_Iio
      refine ⟨Iio c, ?_⟩
      sorry
    · sorry
    · sorry
  · rcases hb₂ with h₂_Iio | h₂_Ioo | h₂_Ioi
    · sorry
    · sorry
    · sorry
  · rcases hb₂ with h₂_Iio | h₂_Ioo | h₂_Ioi
    · sorry
    · sorry
    · sorry


-- example : ∀ b₁ ∈ @B α _, ∀ b₂ ∈ B, ∀ x ∈ b₁ ∩ b₂, ∃ b ∈ B, x ∈ b ∧ b ⊆ b₁ ∩ b₂ :=
-/
