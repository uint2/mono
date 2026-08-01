import Munkres.Mathlib.AccPt.Basic
import Munkres.Defs.Countable
import Munkres.Defs.IsBasisAt

open Filter Set Munkres
open scoped Topology

universe u

variable {α : Type u} [TopologicalSpace α]
  {A : Set α} {x : α}

theorem AccPt.exists_tendsto [h₁ : FirstCountableTopology α] {A : Set α} {x : α}
  : AccPt x (𝓟 A) → ∃ (f : ℕ → α), (∀ n, f n ∈ A) ∧ Tendsto f atTop (𝓝 x)
  := by --
  intro hx
  rw [AccPt.iff] at hx
  rw [FirstCountableTopology.iff] at h₁
  specialize h₁ x
  obtain ⟨β, hβ_countable, hβx⟩ := h₁
  haveI : Countable β := hβ_countable
  obtain ⟨B, hB_anti, hB⟩ := hβx.exists_antitone_eq_range
  let hδ (n : ℕ) := hx (B n) (hB.isOpen' ⟨n, rfl⟩) (hB.mem' ⟨n, rfl⟩)
  let f (n : ℕ) : α := (hδ n).some
  use f
  refine ⟨?_, ?_⟩
  · intro n
    obtain ⟨hfB : f n ∈ B n, hfA : f n ∈ A \ {x}⟩ := (hδ n).some_mem
    exact hfA.1
  · rw [tendsto_atTop_nhds]
    intro U hxU hU
    obtain ⟨b, ⟨N, heq⟩, hbU⟩ := hB.exists_mem_subset' hU hxU
    subst heq
    use N
    intro n hn
    obtain ⟨hfB : f n ∈ B n, hfA : f n ∈ A \ {x}⟩ := (hδ n).some_mem
    exact hbU <| hB_anti hn hfB -- ∎
