import Mathlib.Algebra.Group.Defs
import Mathlib.SetTheory.Cardinal.Finite
import Mathlib.Basic.ExistsUnique
import Mathlib.Data.ZMod.Defs
import Mathlib.GroupTheory.OrderOfElement

/-! Dummit & Foote, Chapter 1, Section 1. Basic Axioms and Examples. -/

universe u

variable {G : Type u}

variable [Group G]

--* Chapter 1, Proposition 1.
theorem no_op : 0 = 0 := rfl

-- If a and b are two group elements, then there exists a unique x such that
-- ax = b.
lemma l₀ (a b : G)
  : ∃! x, a * x = b
  := by
  refine ⟨a⁻¹ * b, ?_, ?_⟩
  · change a * (a⁻¹ * b) = b
    rw [<-mul_assoc, mul_inv_cancel, one_mul]
  · intro y h
    replace h : a⁻¹ * (a * y) = a⁻¹ * b := by rw [h]
    rw [<-mul_assoc, inv_mul_cancel, one_mul] at h
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


/-
The order of an element x of a group is the smallest (n : ℕ) such that

  x * x * ... * x (n times) = e

where e is the identity element, and * is the group operation. In Mathlib we
denote this as
  · x ^ n for Group, and
  · n • x (\smul) for AddGroup.
-/


/--
The order of a Group G. If G is infinite, then `orderOfGroup G = 0`.
-/
noncomputable def orderOfGroup (G : Type u) [Group G] : ℕ := Nat.card G
/--
The order of an AddGroup G. If G is infinite, then `orderOfAddGroup G = 0`.
-/
noncomputable def orderOfAddGroup (G : Type u) [AddGroup G] : ℕ := Nat.card G

example : orderOfAddGroup (ZMod 2) = 2
  := by --
  dsimp [orderOfAddGroup]
  exact Nat.card_zmod 2 -- ∎

example : addOrderOf (2 : ZMod 4) = 2
  := by --
  rw [addOrderOf_eq_iff Nat.zero_lt_two]
  refine ⟨?_, ?_⟩
  · simp only [succ_nsmul]
    rw [zero_nsmul, zero_add]
    rfl
  · intro x hx hx₀
    simp only [nsmul_eq_mul]
    replace hx : x = 1 := Nat.eq_of_le_of_lt_succ hx₀ hx
    subst hx
    rw [Nat.cast_one, one_mul]
    exact Ne.symm (not_eq_of_beq_eq_false rfl) -- ∎
