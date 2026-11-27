/-
Copyright (c) 2026 Nguyen Vu Khang. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Nguyen Vu Khang
-/
module

public import Mathlib.Algebra.Group.Defs

/-! Something. -/

universe u

/-- A custom group builder from first principles -/
@[to_additive, instance_reducible]
private def CustomGroup (G : Type u) (φ : G → G → G)
  (identity : G)
  (inv : G → G)
  (one_mul : ∀ a : G, φ identity a = a)
  (inv_mul_cancel : ∀ a : G, φ (inv a) a = identity)
  (mul_assoc : ∀ a b c, φ (φ a b) c = φ a (φ b c))
  : Group G := by
  have l₀ (x : G) : φ x x = x → x = identity := by
    intro h
    rw [<-one_mul x, <-inv_mul_cancel x, mul_assoc, h]
  have mul_inv_cancel (a : G) : φ a (inv a) = identity := by
    refine l₀ (φ a (inv a)) ?_
    rw [mul_assoc, <-mul_assoc (inv a) a (inv a), inv_mul_cancel, one_mul]
  have mul_one (a : G) : φ a identity = a := by
    rw [<-inv_mul_cancel a, <-mul_assoc, mul_inv_cancel, one_mul]
  let hd : DivInvMonoid G := {
    toMonoid := {
      one := identity
      mul := φ
      one_mul := one_mul
      mul_one := mul_one
      mul_assoc := mul_assoc
    }
    toInv := { inv := inv }
  }
  exact {
    toDivInvMonoid := hd
    inv_mul_cancel := inv_mul_cancel
  }

/-- Checks if a custom operation forms a Group. -/
@[to_additive]
public theorem formsGroup (G : Type u) (φ : G → G → G)
  (identity : G)
  (inv : G → G)
  (one_mul : ∀ a : G, φ identity a = a)
  (inv_mul_cancel : ∀ a : G, φ (inv a) a = identity)
  (mul_assoc : ∀ a b c, φ (φ a b) c = φ a (φ b c))
  : Nonempty (Group G) := by
  refine Nonempty.intro ?_
  exact CustomGroup G φ identity inv one_mul inv_mul_cancel mul_assoc

-- public theorem noInverseNoGroup (G : Type u)
