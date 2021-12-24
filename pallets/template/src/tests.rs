use super::*;
use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		)
	})
}

#[test]
fn create_claim_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
			TemplateModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(TemplateModule::transfer_claim(Origin::signed(1), 2, claim.clone()));

		let (owner, _) = Proofs::<Test>::get(&claim).unwrap();

		assert_eq!(owner, Some(2).unwrap());
	})
}

#[test]
fn transfer_claim_works_when_claim_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));

		assert_noop!(
			TemplateModule::transfer_claim(Origin::signed(2), 1, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn input_length_limit() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2];

		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
	})
}

#[test]
fn input_when_lengtn_max_limit() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3];

		assert_noop!(
			TemplateModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::MaxLengthLimit
		);
	})
}
