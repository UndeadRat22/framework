use typenum::marker_traits::Unsigned;
use types::config::Config;
use types::primitives::{Epoch, Slot};

pub fn epoch_of_slot<C: Config>(slot: Slot) -> Epoch {
    slot / C::SlotsPerEpoch::to_u64()
}

pub fn compute_start_slot_of_epoch<C: Config>(epoch: Epoch) -> Slot {
    epoch * C::SlotsPerEpoch::to_u64()
}

pub fn compute_activation_exit_epoch<C: Config>(epoch: Epoch) -> Epoch {
    epoch + 1 + C::activation_exit_delay()
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::config::MainnetConfig;

    #[test]
    fn test_epoch_of_slot() {
        let expected_epoch = 2 as u64;
        let calculated_epoch = epoch_of_slot::<MainnetConfig>(17 as u64);
        assert_eq!(calculated_epoch, expected_epoch);
    }

    #[test]
    fn test_compute_start_slot_of_epoch() {
        let bs: BeaconState<MainnetConfig> = BeaconState {
            slot: 0,
            ..BeaconState::default()
        };
        assert_eq!(
            compute_start_slot_of_epoch<MainnetConfig>(&bs, 10_u64),
            <MainnetConfig as Config>::SlotsPerEpoch::to_u64() * 10_u64
        )
    }

    #[test]
    fn compute_activation_exit_epoch() {
        assert_eq!(compute_activation_exit_epoch<MainnetConfig>(0), 5);
    }
}