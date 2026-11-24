module

import Onigiri.DnF.Chapter1.Defs

import Mathlib.Basic.ExistsUnique
import Mathlib.Tactic.Lemma

open PendanticOnigiri

universe u

variable {G : Type u}

variable [Group G]

--* Chapter 1, Proposition 1.
theorem no_op : 0 = 0 := rfl

section
open Group

-- If a and b are two group elements, then there exists a unique x such that
-- ax = b.
lemma l₀ (a b : G)
  : ∃! x, a * x = b
  := by
  refine ⟨a⁻¹ * b, ?_, ?_⟩
  · change a * (a⁻¹ * b) = b
    rw [<-mul_assoc]
    rw [mul_inv_cancel]
    rw [one_mul]
  · intro y h
    replace h : a⁻¹ * (a * y) = a⁻¹ * b := by rw [h]
    rw [<-mul_assoc] at h
    rw [inv_mul_cancel] at h
    rw [one_mul] at h
    exact h

--* Chpt 1, Prop 2, Part (1). Group's (left) cancellation law.
theorem proposition_2_1 (a u v : G)
  : a * u = a * v → u = v
  := by --
  intro h
  replace h : a⁻¹ * (a * u) = a⁻¹ * (a * v) := by rw [h]
  rw [<-mul_assoc, <-mul_assoc] at h
  rw [inv_mul_cancel] at h
  rw [one_mul, one_mul] at h
  exact h -- ∎

--* Chpt 1, Prop 2, Part (2). Group's (right) cancellation law.
theorem proposition_2_2 (b u v : G)
  : u * b = v * b → u = v
  := by --
  intro h
  replace h : u * b * b⁻¹ = v * b * b⁻¹ := by rw [h]
  rw [mul_assoc, mul_assoc] at h
  rw [mul_inv_cancel] at h
  rw [mul_one, mul_one] at h
  exact h -- ∎

end section
