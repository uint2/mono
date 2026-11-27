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
import Onigiri.Tools.AssertGoal

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

/--
For all n > 1, ℤ/nℤ is not a group under multiplication of residue classes.
This is equivalent to saying that the set of all such groups is empty.
-/
theorem problem_5 : ∀ n > 1, IsEmpty { g : Group (ZMod n) // g.mul = (· * ·) }
  := by --
  intro n hn
  -- We shall prove that if such a group exists, its multiplication cannot be φ.
  refine Subtype.isEmpty_of_false ?_
  intro Γ
  -- We give a name, φ, to the multiplication of reside classes of ℤ/nℤ.
  let φ : ZMod n → ZMod n → ZMod n := (· * ·)
  -- We assume on the contrary that the multiplication on G __is__ φ.
  by_contra (hφ : Γ.mul = φ)
  -- We shall use these two statements to create the contradiction:
  suffices h
    : (∀ a : ZMod n, a * 0 ≠ 1)      --> intent: 0 * 0 ≠ 1
    ∧ (∀ a : ZMod n, ∃ b, b * a = 1) --> intent: 0 * 0 = 1
    by
    obtain ⟨h₁, hInv⟩ := h
    specialize hInv 0
    obtain ⟨b, hInv⟩ := hInv -- b is 0's inverse element.
    exact (h₁ b) hInv
  refine ⟨?_, ?_⟩
  -- We use the fact that we're in ℤ/nℤ with n > 1 to show that every element
  -- operated on zero yields a non-1 result.
  · intro a
    assert_goal φ a 0 ≠ 1 -- we're on the φ channel.
    rw [mul_zero]
    have : NeZero (1 : ZMod n) := by
      have : Nontrivial (ZMod n) := by
        rw [ZMod.nontrivial_iff]
        exact ne_of_gt hn
      exact NeZero.one
    exact zero_ne_one
  -- The assumption that ZMod n is a Group implies that every element has an
  -- inverse.
  · intro a
    change ∃ b, φ b a = 1 -- again, we're starting on the φ channel.
    --
    -- At this point, since a ∈ ℤ/nℤ, invoking "a⁻¹" will canonically call
    -- "@Inv.inv (ZMod n) (ZMod.instInv n) a" which is not what we want.
    -- Instead, we want to make use of the fact that ℤ/nℤ is assumed to be a
    -- group and use that to create the inverse.
    --
    -- Note that since the inverse is created from the assumption (Γ) that ℤ/nℤ
    -- is a group, then the `inv_mul_cancel` theorem invoked shall also come
    -- from Γ. Hence, we have to move everything over to being in terms of Γ.mul
    refine ⟨Γ.inv a, ?_⟩
    -- switch over from φ to Γ.mul.
    rw [<-hφ]
    assert_goal Γ.mul (Γ.inv a) a = (1 : ZMod n) -- now we're working with Γ.mul
    -- One last thing to switch over: the 1 on the RHS.
    have h₁ : Γ.one = (1 : ZMod n) := by
      have mul_one : Γ.mul (1 : ZMod n) Γ.one = (1 : ZMod n) := Γ.mul_one (1 : ZMod n)
      rw [<-mul_one, hφ]
      change Γ.one = 1 * Γ.one
      exact (one_mul Γ.one).symm
    rw [<-h₁]
    exact Γ.inv_mul_cancel a -- ∎

end Problem5

namespace Problem6

private def Q₀ : Set ℚ := { q | Even q.den ∨ q = 0 }
private def Q₁ : Set ℚ := { q | Odd q.den ∨ q = 0 }

/--
If a is odd, then all factors of a are odd. This is because if there were any
factors of a that is even, then a would be even.
-/
lemma problem_6a₁ {a d : ℕ} : Odd a → d ∣ a → Odd (a / d)
  := by --
  intro ha₁ ⟨k, hk⟩
  have hd₀ : d ≠ 0 := by
    by_contra hd₀
    subst hd₀
    rw [zero_mul] at hk
    subst hk
    exact Nat.not_odd_zero ha₁
  have hDiv : k = a / d := Nat.eq_div_of_mul_eq_right hd₀ hk.symm
  rw [<-hDiv]
  by_contra hk₁ -- suppose on the contrary that k (factor of a) is even.
  rw [Nat.not_odd_iff_even] at hk₁
  have ha₂ : 2 ∣ a := by
    rw [hk]
    exact Nat.dvd_mul_left_of_dvd hk₁.two_dvd d
  rw [<-even_iff_two_dvd] at ha₂
  rw [<-Nat.not_odd_iff_even] at ha₂
  exact ha₂ ha₁ -- ∎

theorem problem_6a : Nonempty { g : AddSubgroup ℚ // g.carrier = Q₁ }
  := by --
  refine Nonempty.intro ⟨?_, ?_⟩
  · exact {
      carrier := Q₁
      zero_mem' := Or.inr rfl
      add_mem' {a b} ha hb := by
        rcases ha with ha | ha₀ <;> rcases hb with hb | hb₀
        · if h₀ : a + b = 0 then rw [h₀]; refine Or.inr rfl else
          refine Or.inl ?_
          rw [Rat.den_add]
          let n₁ := a.num
          let n₂ := b.num
          let d₁ := a.den
          let d₂ := b.den
          change Odd (d₁ * d₂ / (n₁ * ↑d₂ + n₂ * ↑d₁).natAbs.gcd (d₁ * d₂))
          have hdvd : (n₁ * ↑d₂ + n₂ * ↑d₁).natAbs.gcd (d₁ * d₂) ∣ d₁ * d₂ := by
            exact Nat.gcd_dvd_right (n₁ * ↑d₂ + n₂ * ↑d₁).natAbs (d₁ * d₂)
          exact problem_6a₁ (ha.mul hb) hdvd
        · rw [hb₀, add_zero]
          exact Or.inl ha
        · rw [ha₀, zero_add]
          exact Or.inl hb
        · rw [ha₀, hb₀, zero_add]
          exact Or.inr rfl
      neg_mem' {a} ha := by
        cases ha with
        | inl ha => exact Or.inl (odd_iff_exists_bit1.mpr ha)
        | inr ha =>
          refine Or.inr ?_
          rw [ha]
          exact Rat.neg_zero
    }
  · rfl -- ∎

theorem problem_6b : IsEmpty { g : AddSubgroup ℚ // g.carrier = Q₀ }
  := by --
  refine Subtype.isEmpty_of_false ?_
  intro G
  by_contra (hCarrier : G.carrier = Q₀)
  have hQ₀ (a : ℚ) : a ∈ Q₀ ↔ a ∈ G := by
    rw [<-G.mem_carrier, hCarrier]
  let a : ℚ := 6⁻¹
  have ha : a ∈ G := by
    change a ∈ G.carrier
    rw [hCarrier]
    refine Or.inl ?_
    have h₆ : a.den = 6 := by
      rw [Rat.den_inv]
      have h₆ : Rat.num 6 ≠ 0 := Ne.symm (OfNat.zero_ne_ofNat 6)
      rw [ite_eq_right h₆]
      rfl
    rw [h₆, Nat.even_iff]
  have h₃ : a + a = 3⁻¹ := by
    dsimp only [a]
    rw [<-two_mul]
    refine Eq.symm (Rat.inv_eq_of_mul_eq_one ?_)
    rw [<-mul_assoc]
    have h₆ : (3 : ℚ) * 2 = 6 := by grind only
    rw [h₆, Rat.mul_inv_cancel]
    exact Rat.abs_pos_iff.mp rfl
  have hIn : a + a ∈ G := add_mem ha ha
  have hOut : 3⁻¹ ∉ G := by
    rw [<-hQ₀]
    dsimp only [Q₀]
    rw [Set.notMem_ofPred_iff]
    push Not
    refine ⟨?_, ?_⟩
    · have : (3 : ℚ)⁻¹.den = 3 := by
        rw [Rat.den_inv]
        have h₆ : Rat.num 3 ≠ 0 := Ne.symm (OfNat.zero_ne_ofNat 3)
        rw [ite_eq_right h₆]
        rfl
      rw [this, Nat.odd_iff]
    · exact inv_ne_zero (Rat.abs_pos_iff.mp rfl)
  rw [h₃] at hIn
  exact hOut hIn -- ∎

theorem problem_6c : IsEmpty { g : AddSubgroup ℚ // g.carrier = { x | |x| < 1 } }
  := by --
  refine Subtype.isEmpty_of_false ?_
  intro G
  let S₁ := { x : ℚ | |x| < 1 }
  by_contra (hCarrier : G.carrier = S₁)
  have hS₁ (a : ℚ) : a ∈ S₁ ↔ a ∈ G := 
    Iff.of_eq (congrFun (congrArg Membership.mem hCarrier.symm) a)
  replace hS₁ (a : ℚ) : a ∈ S₁ ↔ a ∈ G := by
    rw [<-G.mem_carrier, hCarrier]
  let x : ℚ := 3 / 4
  have hx₁ : x < 1 := (div_lt_one₀ rfl).mpr rfl
  have hx₀ : 0 < x := div_pos rfl rfl
  have hx₁' : |x| < 1 := abs_lt.mpr ⟨hx₀.trans' rfl, hx₁⟩
  have hxG : x ∈ G := by
    rw [<-hS₁]
    exact hx₁'
  have hMem : x + x ∈ G := G.add_mem hxG hxG
  have hNotMem : x + x ∉ G := by
    rw [<-hS₁]
    dsimp only [x, S₁]
    have hx₂ : x + x = 3 / 2 := by grind only
    rw [hx₂, Set.notMem_ofPred_iff]
    have hle : (1 : ℚ) ≤ |3 / 2| := by
      grind =>
        instantiate only [= abs_mul]
        instantiate only [= abs.eq_1]
        instantiate only [= max_def]
        linarith
    exact hle.not_gt
  exact hNotMem hMem -- ∎

theorem problem_6d : IsEmpty { g : AddSubgroup ℚ // g.carrier = { x | x = 0 ∨ 1 ≤ |x| } }
  := by --
  refine Subtype.isEmpty_of_false ?_
  intro G
  let S₁ := { x : ℚ | x = 0 ∨ 1 ≤ |x| }
  by_contra (hCarrier : G.carrier = S₁)
  have hS₁ (a : ℚ) : a ∈ S₁ ↔ a ∈ G := 
    Iff.of_eq (congrFun (congrArg Membership.mem hCarrier.symm) a)
  replace hS₁ (a : ℚ) : a ∈ S₁ ↔ a ∈ G := by
    rw [<-G.mem_carrier, hCarrier]
  let x : ℚ := 8 / 3
  let y : ℚ := 7 / 3
  have hxG : x ∈ G := by
    rw [<-hS₁, Set.mem_ofPred]
    refine Or.inr (le_abs.mpr (Or.inl ?_))
    exact (one_le_div₀ rfl).mpr rfl
  have hyG : y ∈ G := by
    rw [<-hS₁, Set.mem_ofPred]
    refine Or.inr (le_abs.mpr (Or.inl ?_))
    exact (one_le_div₀ rfl).mpr rfl
  have hMem : x - y ∈ G := G.sub_mem hxG hyG
  have hNotMem : x - y ∉ G := by
    rw [<-hS₁, Set.notMem_ofPred_iff]
    push Not
    have hxy : x - y = 1 / 3 := by grind only
    rw [hxy]
    refine ⟨?_, ?_⟩
    · exact one_div_ne_zero (Rat.abs_pos_iff.mp rfl)
    · have hlt : (1 : ℚ) < 3 := rfl
      have : 0 < (1 : ℚ) / 3 := one_div_pos.mpr rfl
      rw [abs_of_pos this]
      exact (div_lt_one₀ hlt).mpr hlt
  exact hNotMem hMem -- ∎

end Problem6
