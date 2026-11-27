module

import Mathlib.Tactic.Lemma

universe u

public section
namespace PendanticOnigiri

class Group (G : Type u) extends One G, Mul G, Inv G where
  one_mul (a : G) : 1 * a = a
  inv_mul_cancel (a : G) : a⁻¹ * a = 1
  /-- The group operation is associative -/
  mul_assoc (a b c : G) : a * b * c = a * (b * c)

variable {G : Type u} [Group G]

private lemma Group.l₀ (x : G) : x * x = x → x = 1 := by
  intro h
  rw [<-one_mul x, <-inv_mul_cancel x, mul_assoc, h]

theorem Group.mul_inv_cancel (a : G) : a * a⁻¹ = 1
  := by --
  refine l₀ (a * a⁻¹) ?_
  rw [mul_assoc, <-mul_assoc a⁻¹ a a⁻¹, inv_mul_cancel, one_mul] -- ∎

theorem Group.mul_one (a : G) : a * 1 = a
  := by --
  rw [<-inv_mul_cancel a, <-mul_assoc, mul_inv_cancel, one_mul] -- ∎

theorem Group.left_inv_eq_right_inv {a b c : G} (hba : b * a = 1) (hac : a * c = 1)
  : b = c
  := by --
  rw [<-one_mul c, <-hba, mul_assoc, hac, mul_one] -- ∎

theorem Group.mul_inv_rev (a b : G) : (a * b)⁻¹ = b⁻¹ * a⁻¹
  := by --
  have h : (a * b)⁻¹ * (a * b) = 1 := inv_mul_cancel (a * b)
  rw [<-mul_assoc] at h
  replace h : (a * b)⁻¹ * a * b * b⁻¹ = 1 * b⁻¹ := congrArg (· * b⁻¹) h
  rw [mul_assoc, mul_inv_cancel, mul_one, one_mul] at h
  replace h : (a * b)⁻¹ * a * a⁻¹ = b⁻¹ * a⁻¹ := congrArg (· * a⁻¹) h
  rw [mul_assoc, mul_inv_cancel, mul_one] at h
  exact h -- ∎

theorem Group.inv_eq_of_mul_eq_one_right (a b : G) : a * b = 1 → a⁻¹ = b
  := by --
  intro h
  replace h : a⁻¹ * (a * b) = a⁻¹ * 1 := congrArg (a⁻¹ * ·) h
  rw [mul_one] at h
  rw [<-mul_assoc, inv_mul_cancel, one_mul] at h
  exact h.symm -- ∎

end PendanticOnigiri
end section
