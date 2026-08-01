import Mathlib.Data.Set.Countable
import Mathlib.Order.OmegaCompletePartialOrder
import Mathlib.Topology.Basic

import Munkres.Defs.Basic

namespace Munkres

universe u

variable {α : Type u} [TopologicalSpace α] {B : Set (Set α)} {x : α}

structure IsBasisAt [TopologicalSpace α] (B : Set (Set α)) (x : α) where
  isOpen' {b} : b ∈ B → IsOpen b
  mem' {b} : b ∈ B → x ∈ b
  exists_mem_subset' {u} : IsOpen u → x ∈ u → ∃ b ∈ B, b ⊆ u

lemma IsBasisAt.nhds' (hB : IsBasisAt B x) {b : Set α} : b ∈ B → b ∈ nhds' x
  := by --
  intro hb
  exact ⟨hB.isOpen' hb, hB.mem' hb⟩ -- ∎

theorem IsBasisAt.nonempty (hB : IsBasisAt B x) : B.Nonempty
  := by --
  obtain ⟨_, hb, _⟩ := hB.exists_mem_subset' isOpen_univ trivial
  exact Set.nonempty_of_mem hb -- ∎

theorem IsBasisAt.exists_eq_range [Countable B] (hB : IsBasisAt B x)
  : ∃ (f : ℕ → Set α), B = Set.range f
  := by --
  have : B.Countable := by
    rw [<-Set.countable_coe_iff]
    infer_instance
  exact this.exists_eq_range hB.nonempty -- ∎

-- Note that the output `Set.range f` is countable just because ℕ is countable.
theorem IsBasisAt.exists_antitone_eq_range [Countable B] (hB : IsBasisAt B x)
  : ∃ (f : ℕ → Set α), Antitone f ∧ IsBasisAt (Set.range f) x
  := by --
  obtain ⟨f, hf⟩ := hB.exists_eq_range
  have hf' (i : ℕ) : f i ∈ B := by rw [hf]; exact ⟨i, rfl⟩
  let φ (n : ℕ) : Set α := ⋂ i ≤ n, f i
  have hφ : Antitone φ := by
    refine antitone_nat_of_succ_le ?_
    intro n
    dsimp only [φ]
    conv => rhs; simp only [<-Nat.lt_add_one_iff]
    rw [Set.biInter_le]
    let A := (⋂ j < n + 1, f j)
    change A ∩ f (n + 1) ⊆ A
    exact Set.inter_subset_left
  refine ⟨φ, hφ, ?_⟩
  exact {
    isOpen' := by
      intro b ⟨n, heq⟩; subst heq
      dsimp only [φ]
      change IsOpen (⋂ i ≤ n, f i)
      simp only [<-Nat.lt_add_one_iff, <-Finset.mem_range]
      refine isOpen_biInter_finset ?_
      intro i _
      exact hB.isOpen' (hf' i)
    mem' := by
      intro b ⟨n, heq⟩; subst heq
      rw [Set.mem_iInter₂]
      intro i _
      exact hB.mem' (hf' i)
    exists_mem_subset' := by
      intro U hU hxU
      obtain ⟨b, hbB, hbU⟩ := hB.exists_mem_subset' hU hxU
      rw [hf] at hbB
      obtain ⟨i, hi⟩ := hbB
      subst hi
      have : φ i ⊆ f i := by
        dsimp only [φ]
        rw [Set.biInter_le]
        exact Set.inter_subset_right
      exact ⟨φ i, ⟨i, rfl⟩, this.trans hbU⟩
  } -- ∎

/-- Definition of a countable basis, as given in Munkres. -/
structure IsCountableBasisAt [TopologicalSpace α] (B : Set (Set α)) (x : α)
  extends Countable B, IsBasisAt B x where

section IsCountableBasisAt
variable (hB : IsCountableBasisAt B x)

include hB in
theorem IsCountableBasisAt.countable : B.Countable
  := by --
  exact hB.toCountable -- ∎

include hB in
theorem IsCountableBasisAt.exists_eq_range
  : ∃ (f : ℕ → Set α), B = Set.range f
  := by --
  exact hB.countable.exists_eq_range hB.toIsBasisAt.nonempty -- ∎

theorem IsCountableBasisAt.exists_antitone_eq_range (hB : IsCountableBasisAt B x)
  : ∃ (f : ℕ → Set α), Antitone f ∧ IsCountableBasisAt (Set.range f) x
  := by --
  obtain ⟨f, hf⟩ := hB.exists_eq_range
  let φ (n : ℕ) : Set α := ⋂ i ≤ n, f i
  have hφ : Antitone φ := by
    refine antitone_nat_of_succ_le ?_
    intro n
    dsimp only [φ]
    conv => rhs; simp only [<-Nat.lt_add_one_iff]
    rw [Set.biInter_le]
    let A := (⋂ j < n + 1, f j)
    change A ∩ f (n + 1) ⊆ A
    exact Set.inter_subset_left
  have hf₂ {i : ℕ} : f i ∈ B := by
    rw [hf]
    exact Set.mem_range_self i
  refine ⟨φ, hφ, Set.countable_range φ, ?_⟩
  refine {
    isOpen' {b} := by
      intro ⟨n, heq⟩
      subst heq
      dsimp only [φ]
      simp only [<-Nat.lt_add_one_iff, <-Finset.mem_range]
      exact isOpen_biInter_finset fun _ _ ↦ hB.isOpen' hf₂
    mem' {b} := by
      intro ⟨n, heq⟩
      subst heq
      dsimp only [φ]
      simp only [<-Nat.lt_add_one_iff, <-Finset.mem_range]
      rw [Set.mem_iInter₂]
      exact fun _ _ ↦ hB.mem' hf₂
    exists_mem_subset' {U} hU hxU := by
      obtain ⟨b, hbB, hbU⟩ := hB.exists_mem_subset' hU hxU
      rw [hf] at hbB
      obtain ⟨n, heq⟩ := hbB
      subst heq
      have : φ n ⊆ f n := by
        dsimp only [φ]
        rw [Set.biInter_le]
        exact Set.inter_subset_right
      exact ⟨φ n, Set.mem_range_self n, this.trans hbU⟩
  } -- ∎

end IsCountableBasisAt

end Munkres
