import Mathlib.Topology.EMetricSpace.Lipschitz
import Mathlib.Topology.MetricSpace.Pseudo.Defs

open NNReal

universe u v

variable {α : Type u} {β : Type v} [PseudoMetricSpace α] [PseudoMetricSpace β]

-- To match Mathlib's `AntilipschitzWith.le_mul_nndist`
theorem LipschitzWith.le_mul_nndist {K : ℝ≥0} {f : α → β}
  : LipschitzWith K f → ∀ x y, nndist (f x) (f y) ≤ K * nndist x y
  := by --
  intro hf x y
  rw [<-edist_le_coe, ENNReal.coe_mul, coe_nnreal_ennreal_nndist]
  exact hf x y -- ∎

-- To match Mathlib's `AntilipschitzWith.le_mul_dist`
theorem LipschitzWith.le_mul_dist {K : ℝ≥0} {f : α → β}
  : LipschitzWith K f → ∀ x y, dist (f x) (f y) ≤ K * dist x y
  := by --
  intro hf x y
  exact mod_cast le_mul_nndist hf x y -- ∎
