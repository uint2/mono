module

import Onigiri.DnF.Chapter1.Section1.Pedantic

open PendanticOnigiri

universe u

/--
This section seeks to reconcile our custom definition of a Group with the one
shown in Dummit and Foote.
-/

class DummitAndFooteGroup (G : Type u) extends Mul G where
  /-- identity element -/
  eye : G
  eye_mul (a : G) : eye * a = a
  mul_eye (a : G) : a * eye = a
  left_inv (a : G) : ∃ b : G, b * a = eye
  right_inv (a : G) : ∃ b : G, a * b = eye
  mul_assoc (a b c : G) : a * b * c = a * (b * c)

@[instance_reducible]
private def fwd (G : Type u) [Group G] : DummitAndFooteGroup G := {
  eye := 1
  eye_mul := Group.one_mul
  mul_eye := Group.mul_one
  mul_assoc := Group.mul_assoc
  left_inv a := ⟨a⁻¹, Group.inv_mul_cancel a⟩
  right_inv a := ⟨a⁻¹, Group.mul_inv_cancel a⟩
}

@[instance_reducible]
private noncomputable def bwd (G : Type u) [DummitAndFooteGroup G] : Group G := {
  one := DummitAndFooteGroup.eye
  inv a := (DummitAndFooteGroup.left_inv a).choose
  one_mul a := DummitAndFooteGroup.eye_mul a
  inv_mul_cancel a := (DummitAndFooteGroup.left_inv a).choose_spec
  mul_assoc := DummitAndFooteGroup.mul_assoc
}
