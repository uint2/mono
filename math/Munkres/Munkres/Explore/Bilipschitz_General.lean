import Mathlib.Topology.MetricSpace.Defs
import Mathlib.Topology.MetricSpace.Antilipschitz
import Mathlib.Topology.MetricSpace.Cauchy
import Mathlib.Topology.UniformSpace.Cauchy

import Munkres.Mathlib
import Munkres.Defs

open Set Filter Topology NNReal

universe u v

variable {α : Type u} {β : Type v} [M₁ : MetricSpace α]
  {f : α ↪ α}
  {r : ℝ≥0} -- bilipschitz ratio
  (hf : LipschitzWith r f) (hf' : AntilipschitzWith r⁻¹ f)

def d := M₁.dist

private local instance M₂ (f : α ↪ α) : MetricSpace α
  := by --
  exact {
    dist x y := d (f x) (f y)
    dist_self _ := dist_self _
    dist_comm _ _ := dist_comm _ _
    dist_triangle _ _ _ := dist_triangle _ _ _
    eq_of_dist_eq_zero h := f.injective <| eq_of_dist_eq_zero h
  } -- ∎

private def ρ (f : α ↪ α) := (M₂ f).dist
example {x y : α} : ρ f x y = d (f x) (f y) := rfl

example {x y : α} : (ρ f) x y ≤ r * d x y := hf.le_mul_dist x y
example {x y : α} : d x y ≤ r⁻¹ * (ρ f) x y := hf'.le_mul_dist x y
