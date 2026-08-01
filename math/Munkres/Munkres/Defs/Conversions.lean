import Mathlib.Topology.MetricSpace.Bounded -- for `Metric.diam`
import Mathlib.Topology.UniformSpace.Cauchy -- for `TotallyBounded`
import Mathlib.Topology.Metrizable.Basic -- for `Metrizable`

namespace Munkres

universe u

variable {α : Type u}

/-- Converts Mathlib's `SeparateNhds` into a statement without 𝓝. -/
protected theorem SeparateNhds.iff [TopologicalSpace α] {A B : Set α} :
  SeparatedNhds A B ↔ ∃ U V, IsOpen U ∧ IsOpen V ∧ A ⊆ U ∧ B ⊆ V ∧ Disjoint U V
  := by --
  rw [separatedNhds_iff_disjoint, Filter.disjoint_iff]
  simp only [mem_nhdsSet_iff_exists]
  constructor
  · intro ⟨s, ⟨U, hU, hAU, hUs⟩, t, ⟨V, hV, hBV, hVt⟩, hd⟩
    exact ⟨U, V, hU, hV, hAU, hBV, Set.disjoint_of_subset hUs hVt hd⟩
  · intro ⟨U, V, hU, hV, hAU, hBV, hd⟩
    exact ⟨U, ⟨U, hU, hAU, le_rfl⟩, V, ⟨V, hV, hBV, le_rfl⟩, hd⟩ -- ∎

end Munkres
