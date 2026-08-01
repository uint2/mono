import Munkres.Defs
import Munkres.Mathlib.Prelude

open Set Filter Topology TopologicalSpace

universe u v

variable {α : Type u} {Λ : Type v} [TopologicalSpace α] (hα : DiscreteTopology α)

example : TopologicalSpace (Λ → α) := Pi.topologicalSpace

example {U : Set (Λ → α)} : IsOpen U := by
  rw [isOpen_pi_iff]
  intro x hxU
  use ∅, fun _ ↦ ∅
  refine ⟨?_, ?_⟩
  · intro x h
    simp only [Finset.notMem_empty] at h
  · rw [Finset.coe_empty]
    rw [Set.empty_pi]
    -- rw [Set.pi_univ]
    sorry


-- inductive Y
--   | a
--   | b
-- open Y

private def Y : Set ℕ := {0, 1}
private theorem hY₀ : 0 ∈ Y := Set.mem_insert 0 {1}
private theorem hY₁ : 1 ∈ Y := Set.mem_insert_of_mem 0 rfl
private def Y₀ : Y := ⟨0, Set.mem_insert 0 {1}⟩
private def Y₁ : Y := ⟨1, Set.mem_insert_of_mem 0 rfl⟩
private theorem hY_notMem : ∀ n > 1, n ∉ Y
  := by --
  intro n hn
  by_contra h
  exact hn.not_ge <| Nat.le_one_iff_eq_zero_or_eq_one.mpr h -- ∎
private instance hY_nat₀ : OfNat Y 0 := { ofNat := Y₀ }
private instance hY_nat₁ : OfNat Y 1 := { ofNat := Y₁ }
private theorem Y_zero_ne_one : Y₀ ≠ Y₁ := not_eq_of_beq_eq_false rfl

private local instance : Zero Y := Zero.ofOfNat0

private local instance : TopologicalSpace Y := {
  IsOpen _ := True,
  isOpen_univ := trivial,
  isOpen_inter _ _ _ _ := trivial,
  isOpen_sUnion _ _ := trivial
}

example {V : Set (Λ → Y)} : IsOpen V := by
  rw [isOpen_pi_iff]
  change ∀ x ∈ V, ∃ Λ' U, (∀ α ∈ Λ', IsOpen (U α) ∧ x α ∈ U α) ∧ (SetLike.coe Λ').pi U ⊆ V
  intro x hxV
  use ∅, fun _ ↦ ∅
  refine ⟨?_, ?_⟩
  · intro x h
    simp only [Finset.notMem_empty] at h
  · rw [Finset.coe_empty]
    rw [Set.empty_pi]
    sorry


def X : Set (ℝ → Y) := { f | (f ⁻¹' {1}).Countable }
def B : Set (Set (ℝ → Y)) := { p | ∃ (J : Finset ℝ) (u : ℝ → Set Y),
  (∀ α ∈ J, IsOpen (u α)) ∧ p = (SetLike.coe J).pi u }

example : IsTopologicalBasis B := by
  refine isTopologicalBasis_of_isOpen_of_nhds ?_ ?_
  · dsimp only [B]
    intro b ⟨J, u, hu, heq⟩
    subst heq
    exact isOpen_set_pi J.finite_toSet hu
  · intro x U hxU hU
    rw [isOpen_pi_iff] at hU
    specialize hU x hxU
    obtain ⟨J, u, hu, hU⟩ := hU
    let v : Set (ℝ → Y) := (SetLike.coe J).pi u
    refine ⟨v, ?_, ?_, hU⟩
    · exact ⟨J, u, fun _ _ ↦ trivial, rfl⟩
    · rw [Set.mem_pi]
      intro α hαJ
      exact (hu α hαJ).2



example {f : ℕ → ℝ} : ∃ φ : ℕ ↪o ℕ, Monotone (f ∘ φ) := by
  sorry
  -- refine PartiallyWellOrderedOn.exists_monotone_subseq (fun f ↦ ?_) ?_
  -- · sorry
  -- · sorry

example : IsSeqCompact (Icc (0 : ℝ) 1) := by
  let I := Icc (0 : ℝ) 1
  -- have : Set.PartiallyWellOrderedOn I (· < ·) := by
  --   -- refine partiallyWellOrderedOn_of_wellQuasiOrdered ?_ I
  --   -- refine (wellQuasiOrderedLE_def ℝ).mp ?_
  --   -- refine wellQuasiOrderedLE_iff_wellFoundedLT.mpr ?_
  --   sorry
  -- have : I.IsPWO := by
  --   dsimp only [I]
  --   rw [isPWO_iff_isWF]
  --   let lt : ℝ → ℝ → Prop := (· < ·)
  --   -- have : WellFoundedLT ℝ := by apply?
  --   -- exact Set.IsPWO.of_linearOrder
  --   sorry
  intro f hf
  have : BddAbove (range f) := by
    refine ⟨1, ?_⟩
    intro x ⟨n, heq⟩
    subst heq
    exact (hf n).2

  have : ∃ φ : ℕ ↪o ℕ, Monotone (f ∘ φ) := by
    -- refine WellQuasiOrdered.exists_monotone_subseq ?_ f
    refine IsPWO.exists_monotone_subseq ?_ hf
    sorry
  have : ∃ φ : ℕ → ℕ, StrictMono φ ∧ Monotone (f ∘ φ) := by
    -- apply?
    sorry
  sorry

example : IsSeqCompact X := by
  intro x hx
  have h_countable (n : ℕ) : ((x n) ⁻¹' {1}).Countable := hx n
  let N : Set ℝ := Set.range Nat.cast
  let f (x : ℝ) : Y := indicator N 1 x
  have hf {x : ℝ} : f x = 1 ↔ x ∈ N := by
    constructor
    · intro h
      have : f x ≠ 0 := by rw [h]; exact Y_zero_ne_one.symm
      exact mem_of_indicator_ne_zero this
    · intro h
      exact indicator_of_mem h 1
  refine ⟨f, ?_, ?_⟩
  · dsimp only [X]
    simp only [mem_setOf_eq]
    have : f ⁻¹' {1} ⊆ N := by
      intro x (hx : f x = 1)
      rw [hf] at hx
      exact hx
    exact (countable_range Nat.cast).mono this
  · simp only [tendsto_atTop_nhds]
    let φ (n : ℕ) : ℕ := n
    refine ⟨φ, fun _ _ ↦ (·), ?_⟩
    intro U hfU hU
    -- rw? at hfU
    -- simp only [mem_pi_iff]
    -- rw [isOpen_pi_iff] at hU
    -- specialize hU f hfU
    sorry

example {f : ℕ → ℝ → Y} {φ : ℝ → Y} :
  Tendsto f atTop (𝓝 φ) ↔ ∀ (Λ' : Finset ℝ), ∃ N, ∀ n ≥ N, ∀ α ∈ Λ', f n α = φ α
  := by
  rw [tendsto_atTop_nhds]
  constructor
  · intro h Λ'
    let U : Set (ℝ → Y) := (SetLike.coe Λ').pi ({φ ·})
    have hφU : φ ∈ U := fun _ _ ↦ rfl
    have hU : IsOpen U := by
      rw [isOpen_pi_iff]
      intro f hfU
      refine ⟨Λ', ({φ ·}), ?_, le_rfl⟩
      intro α hα
      exact ⟨trivial, hfU α hα⟩
    specialize h U hφU hU
    obtain ⟨N, hN⟩ := h
    use N
    intro n hn α hα
    specialize hN n hn
    rw [Set.mem_pi] at hN
    specialize hN α hα
    exact hN
  · intro h U hφU hU
    rw [isOpen_pi_iff] at hU
    specialize hU φ hφU
    obtain ⟨Λ', u, h₁, h₂⟩ := hU
    specialize h Λ'
    obtain ⟨N, hN⟩ := h
    use N
    intro n hn
    specialize hN n hn
    refine h₂ ?_
    rw [Set.mem_pi]
    intro α hα
    specialize hN α hα
    rw [hN]
    exact (h₁ α hα).2
