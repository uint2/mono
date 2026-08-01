import Mathlib.Data.Set.Image

universe u v

variable {X : Type u} {Y : Type v} {s : Set X} {f : X → Y}

/-- Saturated set with respect to a function, as defined in Munkres' Topolgoy. -/
def IsSaturated (s : Set X) (f : X → Y) : Prop := s = f⁻¹' (f '' s)

protected theorem IsSaturated.iff : IsSaturated s f ↔ ∀ x, ∀ a ∈ s, f a = f x → x ∈ s
  := by --
  constructor
  · intro h x a ha z
    exact h ▸ (Set.mem_image _ _ _).mp ⟨a, ha, z⟩
  · intro h
    refine Set.ext fun x => ⟨fun hx => ?_, fun ⟨a, f, z⟩ => h x a f z⟩
    exact (Set.mem_image _ _ _).mp ⟨x, hx, rfl⟩ -- ∎

/-- If A ⊆ X is saturated, and we restrict f to A by g := f|_A, then for any
  V ⊆ A, we have g⁻¹'(V) = f⁻¹'(V); that is, the preimages agree on any V ⊆ A. -/
theorem IsSaturated.setRestrict_preimage (h : IsSaturated s f) (V : Set (f '' s))
  : (↑) '' (s.imageFactorization f⁻¹' V) = f⁻¹' ((↑) '' V)
  := by --
  refine Set.ext fun x => ⟨fun hx => ?_, fun hx => ?_⟩
  · obtain ⟨a, ha, hax⟩ := hx
    subst hax
    refine (Set.mem_image Subtype.val V (f a)).mpr ?_
    exact ⟨⟨(f a), ⟨a, a.prop, rfl⟩⟩, ha, rfl⟩
  · obtain ⟨⟨b, hb⟩, hV, hbx⟩ := hx
    have : f x ∈ f '' s := Set.mem_of_eq_of_mem hbx.symm hb
    refine (Set.mem_image Subtype.val (s.imageFactorization f⁻¹' V) x).mpr ?_
    refine ⟨⟨x, h ▸ this⟩, ?_, rfl⟩
    subst hbx
    exact hV -- ∎

/-- If f : X → Y, and A ⊆ X is saturated, then f(A ∩ U) = f(A) ∩ f(U). -/
theorem IsSaturated.inter_eq (h : IsSaturated s f) (t : Set X)
  : f '' (s ∩ t) = f '' s ∩ f '' t
  := by --
  refine Set.Subset.antisymm_iff.mpr ⟨Set.image_inter_subset f s t, ?_⟩
  intro y ⟨⟨a, ha, hay⟩, ⟨u, hu, huy⟩⟩
  have : u ∈ s := IsSaturated.iff.mp h u a ha (hay.trans huy.symm)
  exact huy ▸ Set.mem_image_of_mem f (Set.mem_inter this hu) -- ∎
