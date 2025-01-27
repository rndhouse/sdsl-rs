use anyhow::Result;

#[test]
fn test_cloned_vector_does_not_share_elements() -> Result<()> {
    let iv = sdsl::IntVector::<0>::new(5, 42, Some(64))?;
    let mut iv_clone = iv.clone();
    iv_clone.set(1, 3);

    let result = (iv.get(1), iv_clone.get(1));
    let expected = (42, 3);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_len_gives_number_of_elements() -> Result<()> {
    let iv = sdsl::IntVector::<0>::new(5, 42, Some(64))?;
    let result = iv.len();
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_correct_len_after_resize() -> Result<()> {
    let mut iv = sdsl::IntVector::<0>::new(5, 42, Some(64))?;
    iv.resize(6);

    let result = iv.len();
    let expected = 6;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_resize_truncates_vector() -> Result<()> {
    let mut iv = sdsl::int_vector! {1, 42, 3};
    iv.resize(2);

    let result: Vec<_> = iv.iter().collect();
    let expected = vec![1, 42];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_correct_bit_size_after_resize() -> Result<()> {
    let mut iv = sdsl::IntVector::<0>::new(5, 42, Some(64))?;
    let before_bit_size = iv.bit_size();

    iv.bit_resize(30);
    let after_bit_size = iv.bit_size();

    let result = (before_bit_size, after_bit_size);
    let expected = (320, 30);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_bit_resize_total_vector_thereby_truncating() -> Result<()> {
    let mut iv = sdsl::int_vector! {1, 42, 3};
    iv.bit_resize(2 * iv.width() as usize);

    let result: Vec<_> = iv.iter().collect();
    let expected = vec![1, 42];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_correct_capacity() -> Result<()> {
    let iv = sdsl::IntVector::<0>::new(5, 42, Some(64))?;
    let result = iv.capacity();
    let expected = 320;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_correct_width() -> Result<()> {
    let iv = sdsl::IntVector::<0>::new(5, 12, Some(28))?;
    let result = iv.width();
    let expected = 28;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_set_width() -> Result<()> {
    let mut iv = sdsl::IntVector::<0>::new(5, 12, Some(28))?;
    let width_before = iv.width();
    iv.set_width(33)?;
    let width_after = iv.width();

    let result = (width_before, width_after);
    let expected = (28, 33);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_set_width_error_on_immutable_width() -> Result<()> {
    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    assert!(iv.set_width(33).is_err());
    Ok(())
}

#[test]
fn test_literal_macro() -> Result<()> {
    let iv = sdsl::int_vector! {1, 42, 3};

    let result = (iv.get(0), iv.get(1), iv.get(2));
    let expected = (1, 42, 3);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    let iv = sdsl::int_vector! {1, 42, 3};

    let result = iv.iter().collect::<Vec<_>>();
    let expected = vec![1, 42, 3];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_into_iter() -> Result<()> {
    let iv = sdsl::int_vector! {1, 42, 3};

    let result = iv.into_iter().collect::<Vec<_>>();
    let expected = vec![1, 42, 3];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_max_size() -> Result<()> {
    let iv = sdsl::int_vector! {1, 42, 3};

    let result = iv.max_size();
    assert!(result > 0);
    Ok(())
}
