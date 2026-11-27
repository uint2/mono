/-
Copyright (c) 2026 Nguyen Vu Khang. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Nguyen Vu Khang
-/
module

import Mathlib.Algebra.Group.Defs
import Mathlib.Data.ZMod.Basic
import Mathlib.Basic.Real.Basic
import Onigiri.DnF.Chapter1.Section1.Bridge

/-! Dummit & Foote, Chapter 1, Section 1.1 Exercises. -/

universe u

variable {G : Type u}

def is_assoc (G : Type u) (f : G → G → G) : Prop := ∀ a b c, f (f a b) c = f a (f b c)
def is_comm (G : Type u) (f : G → G → G) : Prop := ∀ a b, f a b = f b a

namespace Problem1

local syntax "congr_add_assoc " term : tactic
macro_rules
  | `(tactic| congr_add_assoc $c) =>
    `(tactic| (
    repeat rw [add_assoc $c _ _];
    refine congrArg ($c + ·) ?_);
  )

theorem problem_1a : ¬is_assoc ℤ (· - ·)
  := by --
  by_contra h
  specialize h 1 2 3
  change 1 - 2 - 3 = 1 - (2 - 3) at h
  simp only [Int.reduceSub, Int.sub_neg, Int.reduceAdd, reduceCtorEq] at h -- ∎

theorem problem_1b : is_assoc ℝ (fun a b ↦ a + b + a * b)
  := by --
  intro a b c
  change a + b + a * b + c + (a + b + a * b) * c = a + (b + c + b * c) + a * (b + c + b * c)
  congr_add_assoc a
  congr_add_assoc b
  rw [add_comm (a * b) c]
  congr_add_assoc c
  rw [mul_add, mul_add]
  rw [add_mul, add_mul]
  rw [add_comm (b * c) (a * b + (a * c + a * (b * c)))]
  congr_add_assoc (a * b)
  congr_add_assoc (a * c)
  rw [add_comm, mul_assoc] -- ∎

theorem problem_1c : ¬is_assoc ℚ ((· + ·) / 5)
  := by --
  by_contra h
  specialize h 1 0 0
  change (((1 : ℚ) + 0) / 5 + 0) / 5 = (1 + (0 + 0) / 5) / 5 at h
  rw [add_zero, add_zero, add_zero, zero_div, add_zero] at h
  have h₅ : (5 : ℚ) ≠ 0 := Rat.abs_pos_iff.mp rfl
  rw [div_left_inj₀ h₅] at h
  exact (div_ne_one_of_ne (Rat.ne_of_lt rfl)) h -- ∎

theorem problem_1d : is_assoc (ℤ × ℤ) (fun (a, b) (c, d) ↦ (a * d + b * c, b * d))
  := by --
  let φ : (ℤ × ℤ) → (ℤ × ℤ) → (ℤ × ℤ) := fun (a, b) (c, d) ↦ (a * d + b * c, b * d)
  intro (a, b) (c, d) (e, f)
  change φ (a * d + b * c, b * d) (e, f) = φ (a, b) (c * f + d * e, d * f)
  change ((a*d + b*c)*f + b*d*e, b*d*f) = (a*(d*f) + b*(c*f + d*e), b*(d*f))
  rw [<-mul_assoc b d f]
  refine congrArg ((· , b * d * f)) ?_
  rw [add_mul, mul_add]
  rw [<-mul_assoc, add_assoc]
  refine congrArg (a * d * f + ·) ?_
  rw [mul_assoc, mul_assoc] -- ∎

theorem problem_1e : ¬is_assoc (ℚˣ) (· / ·)
  := by --
  by_contra h
  let two : ℚˣ := by
    refine ⟨2, 2⁻¹, ?_, ?_⟩
    · exact Rat.mul_inv_cancel 2 (Rat.abs_pos_iff.mp rfl)
    · exact Rat.inv_mul_cancel 2 (Rat.abs_pos_iff.mp rfl)
  specialize h 1 two two
  change 1 / two / two = 1 / (two / two) at h
  rw [div_self', div_one] at h
  replace h : 1 / two = 1 * two := (mul_eq_of_eq_mul_inv h.symm).symm
  replace h : 1 = 1 * two * two := eq_mul_of_mul_inv_eq h
  rw [one_mul] at h
  have h₀ : two⁻¹ = two := (eq_inv_of_mul_eq_one_left h.symm).symm
  have h₁ : two⁻¹ ≠ two := by
    refine ne_of_lt ?_
    have h₂ : (1 : ℚ) < 2 := one_lt_two
    exact (inv_lt_one_of_one_lt₀ h₂).trans h₂
  exact h₁ h₀ -- ∎

end Problem1

namespace Problem2

theorem problem_2a : ¬is_comm ℤ (· - ·)
  := by --
  by_contra h
  specialize h 1 0
  change 1 - 0 = 0 - 1 at h
  rw [sub_zero, zero_sub] at h
  simp only [Int.reduceNeg, reduceCtorEq] at h -- ∎

theorem problem_2b : is_comm ℝ (fun a b ↦ a + b + a * b)
  := by --
  intro a b
  change a + b + a * b = b + a + b * a
  rw [add_comm a b, mul_comm] -- ∎

theorem problem_2c : is_comm ℚ ((· + ·) / 5)
  := by --
  intro a b
  change (a + b) / 5 = (b + a) / 5
  refine congrArg (· / 5) ?_
  exact Rat.add_comm a b -- ∎

theorem problem_2d : is_comm (ℤ × ℤ) (fun (a, b) (c, d) ↦ (a * d + b * c, b * d))
  := by --
  intro (a, b) (c, d)
  change (a * d + b * c, b * d) = (c * b + d * a, d * b)
  rw [mul_comm d b]
  refine congrArg ((·, b * d)) ?_
  rw [add_comm, mul_comm b c, mul_comm a d] -- ∎

theorem problem_2e : ¬is_comm (ℚˣ) (· / ·)
  := by --
  by_contra h
  let two : ℚˣ := by
    refine ⟨2, 2⁻¹, ?_, ?_⟩
    · exact Rat.mul_inv_cancel 2 (Rat.abs_pos_iff.mp rfl)
    · exact Rat.inv_mul_cancel 2 (Rat.abs_pos_iff.mp rfl)
  specialize h 1 two
  change 1 / two = two / 1 at h
  rw [div_one, one_div] at h
  have h₁ : two⁻¹ ≠ two := by
    refine ne_of_lt ?_
    have h₂ : (1 : ℚ) < 2 := one_lt_two
    exact (inv_lt_one_of_one_lt₀ h₂).trans h₂
  refine h₁ h -- ∎

end Problem2

namespace Problem3

theorem problem_3 {n : ℕ} : is_assoc (ZMod n) (· + ·)
  := by --
  cases n with
  | zero => exact Int.add_assoc
  | succ n =>
  intro a' b' c'
  change a' + b' + c' = a' + (b' + c')
  let a := a'.val; let b := b'.val; let c := c'.val
  rw [<-ZMod.natCast_zmod_val a']
  rw [<-ZMod.natCast_zmod_val b']
  rw [<-ZMod.natCast_zmod_val c']
  change (a : ZMod (n + 1)) + b + c = a + (b + c)
  rw [<-Nat.cast_add]
  rw [<-Nat.cast_add]
  rw [<-Nat.cast_add]
  rw [<-Nat.cast_add]
  rw [ZMod.natCast_eq_natCast_iff']
  rw [Nat.add_assoc] -- ∎

end Problem3

namespace Problem4

theorem problem_4 {n : ℕ} : is_assoc (ZMod n) (· * ·)
  := by --
  cases n with
  | zero => exact Int.mul_assoc
  | succ n =>
  intro a' b' c'
  change a' * b' * c' = a' * (b' * c')
  let a := a'.val; let b := b'.val; let c := c'.val
  rw [<-ZMod.natCast_zmod_val a']
  rw [<-ZMod.natCast_zmod_val b']
  rw [<-ZMod.natCast_zmod_val c']
  change (a : ZMod (n + 1)) * b * c = a * (b * c)
  rw [<-Nat.cast_mul]
  rw [<-Nat.cast_mul]
  rw [<-Nat.cast_mul]
  rw [<-Nat.cast_mul]
  rw [ZMod.natCast_eq_natCast_iff']
  rw [Nat.mul_assoc] -- ∎

end Problem4

namespace Problem5

private def NoGroup (G : Type u) (φ : G → G → G) := IsEmpty { g : Group G // g.mul = φ }

theorem problem_5 : ∀ n > 1, IsEmpty { g : Group (ZMod n) // g.mul = (· * ·) } := by
  intro n hn
  let φ : ZMod n → ZMod n → ZMod n := (· * ·)
  refine Subtype.isEmpty_of_false ?_
  intro G
  by_contra (hMul : G.mul = φ)
  have hMul (a b : ZMod n) : G.mul a b = φ a b := congrFun₂ hMul a b
  have h₀ : Nontrivial (ZMod n) := by
    rw [ZMod.nontrivial_iff]
    exact ne_of_gt hn
  have h' : NeZero (1 : ZMod n) := by
    have : Nontrivial (ZMod n) := by
      rw [ZMod.nontrivial_iff]
      exact ne_of_gt hn
    exact NeZero.one
  have hLeft : ∀ a : ZMod n, a * 0 ≠ 1 := by
    intro a
    rw [mul_zero]
    exact zero_ne_one
  have hRight : ∀ a : ZMod n, ∃ b, b * a = 1 := by
    intro a
    change ∃ b, φ b a = 1
    refine ⟨@Inv.inv (ZMod n) G.toDivInvMonoid.toInv a, ?_⟩
    rw [<-hMul]
    let G' := G.toDivInvMonoid
    have h₁ : G.one = (1 : ZMod n) := by
      have h1 : G.mul (1 : ZMod n) G.one = (1 : ZMod n) := by
        exact G.mul_one _
      rw [<-h1, hMul]
      exact (MulOneClass.one_mul G.one).symm
    rw [<-h₁]
    exact G.inv_mul_cancel _
  specialize hRight 0
  obtain ⟨b, hRight⟩ := hRight
  specialize hLeft b
  refine hLeft hRight

end Problem5
