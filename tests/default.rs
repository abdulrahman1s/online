#[cfg(test)]
use online::check;
use pretty_assertions::assert_eq;

#[async_std::test]
async fn should_work_no_parameters() {
    assert_eq!(check(None).await.is_ok(), true);
}

#[async_std::test]
async fn should_work_timeout() {
    assert_eq!(check(Some(5)).await.is_ok(), true);
}

#[async_std::test]
#[should_panic(expected = "cannot set a 0 duration timeout")]
async fn should_fail_timeout_zero() {
    check(Some(0)).await.unwrap();
}
