#[burn_tensor_testgen::testgen(ad_conv3d)]
mod tests {
    use super::*;
    use burn_tensor::{Shape, module::conv3d, ops::ConvOptions};

    #[test]
    fn test_conv3d_basic() {
        let test = Conv3dTestCase {
            batch_size: 2,
            channels_in: 2,
            channels_out: 2,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 1,
            depth: 4,
            height: 4,
            width: 4,
        };
        let device = Default::default();
        let grads = Grads {
            x: TestTensor::from_floats(
                [
                    [
                        [
                            [
                                [536., 816., 816., 552.],
                                [840., 1278., 1278., 864.],
                                [840., 1278., 1278., 864.],
                                [584., 888., 888., 600.],
                            ],
                            [
                                [912., 1386., 1386., 936.],
                                [1422., 2160., 2160., 1458.],
                                [1422., 2160., 2160., 1458.],
                                [984., 1494., 1494., 1008.],
                            ],
                            [
                                [912., 1386., 1386., 936.],
                                [1422., 2160., 2160., 1458.],
                                [1422., 2160., 2160., 1458.],
                                [984., 1494., 1494., 1008.],
                            ],
                            [
                                [680., 1032., 1032., 696.],
                                [1056., 1602., 1602., 1080.],
                                [1056., 1602., 1602., 1080.],
                                [728., 1104., 1104., 744.],
                            ],
                        ],
                        [
                            [
                                [968., 1464., 1464., 984.],
                                [1488., 2250., 2250., 1512.],
                                [1488., 2250., 2250., 1512.],
                                [1016., 1536., 1536., 1032.],
                            ],
                            [
                                [1560., 2358., 2358., 1584.],
                                [2394., 3618., 3618., 2430.],
                                [2394., 3618., 3618., 2430.],
                                [1632., 2466., 2466., 1656.],
                            ],
                            [
                                [1560., 2358., 2358., 1584.],
                                [2394., 3618., 3618., 2430.],
                                [2394., 3618., 3618., 2430.],
                                [1632., 2466., 2466., 1656.],
                            ],
                            [
                                [1112., 1680., 1680., 1128.],
                                [1704., 2574., 2574., 1728.],
                                [1704., 2574., 2574., 1728.],
                                [1160., 1752., 1752., 1176.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [536., 816., 816., 552.],
                                [840., 1278., 1278., 864.],
                                [840., 1278., 1278., 864.],
                                [584., 888., 888., 600.],
                            ],
                            [
                                [912., 1386., 1386., 936.],
                                [1422., 2160., 2160., 1458.],
                                [1422., 2160., 2160., 1458.],
                                [984., 1494., 1494., 1008.],
                            ],
                            [
                                [912., 1386., 1386., 936.],
                                [1422., 2160., 2160., 1458.],
                                [1422., 2160., 2160., 1458.],
                                [984., 1494., 1494., 1008.],
                            ],
                            [
                                [680., 1032., 1032., 696.],
                                [1056., 1602., 1602., 1080.],
                                [1056., 1602., 1602., 1080.],
                                [728., 1104., 1104., 744.],
                            ],
                        ],
                        [
                            [
                                [968., 1464., 1464., 984.],
                                [1488., 2250., 2250., 1512.],
                                [1488., 2250., 2250., 1512.],
                                [1016., 1536., 1536., 1032.],
                            ],
                            [
                                [1560., 2358., 2358., 1584.],
                                [2394., 3618., 3618., 2430.],
                                [2394., 3618., 3618., 2430.],
                                [1632., 2466., 2466., 1656.],
                            ],
                            [
                                [1560., 2358., 2358., 1584.],
                                [2394., 3618., 3618., 2430.],
                                [2394., 3618., 3618., 2430.],
                                [1632., 2466., 2466., 1656.],
                            ],
                            [
                                [1112., 1680., 1680., 1128.],
                                [1704., 2574., 2574., 1728.],
                                [1704., 2574., 2574., 1728.],
                                [1160., 1752., 1752., 1176.],
                            ],
                        ],
                    ],
                ],
                &device,
            ),
            weight: TestTensor::from_floats(
                [
                    [
                        [
                            [
                                [4590., 6156., 4644.],
                                [6264., 8400., 6336.],
                                [4806., 6444., 4860.],
                            ],
                            [
                                [6696., 8976., 6768.],
                                [9120., 12224., 9216.],
                                [6984., 9360., 7056.],
                            ],
                            [
                                [5454., 7308., 5508.],
                                [7416., 9936., 7488.],
                                [5670., 7596., 5724.],
                            ],
                        ],
                        [
                            [
                                [8046., 10764., 8100.],
                                [10872., 14544., 10944.],
                                [8262., 11052., 8316.],
                            ],
                            [
                                [11304., 15120., 11376.],
                                [15264., 20416., 15360.],
                                [11592., 15504., 11664.],
                            ],
                            [
                                [8910., 11916., 8964.],
                                [12024., 16080., 12096.],
                                [9126., 12204., 9180.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [4590., 6156., 4644.],
                                [6264., 8400., 6336.],
                                [4806., 6444., 4860.],
                            ],
                            [
                                [6696., 8976., 6768.],
                                [9120., 12224., 9216.],
                                [6984., 9360., 7056.],
                            ],
                            [
                                [5454., 7308., 5508.],
                                [7416., 9936., 7488.],
                                [5670., 7596., 5724.],
                            ],
                        ],
                        [
                            [
                                [8046., 10764., 8100.],
                                [10872., 14544., 10944.],
                                [8262., 11052., 8316.],
                            ],
                            [
                                [11304., 15120., 11376.],
                                [15264., 20416., 15360.],
                                [11592., 15504., 11664.],
                            ],
                            [
                                [8910., 11916., 8964.],
                                [12024., 16080., 12096.],
                                [9126., 12204., 9180.],
                            ],
                        ],
                    ],
                ],
                &device,
            ),
            bias: TestTensor::from_floats([128., 128.], &device),
        };
        test.assert_grads(grads);
    }

    // TODO

    #[test]
    fn test_conv3d_complex() {
        let test = Conv3dTestCase {
            batch_size: 1,
            channels_in: 2,
            channels_out: 3,
            kernel_size_1: 2,
            kernel_size_2: 3,
            kernel_size_3: 4,
            padding_1: 1,
            padding_2: 2,
            padding_3: 3,
            stride_1: 1,
            stride_2: 2,
            stride_3: 3,
            dilation_1: 2,
            dilation_2: 3,
            dilation_3: 4,
            groups: 1,
            depth: 5,
            height: 6,
            width: 7,
        };
        let device = Default::default();
        let grads = Grads {
            x: TestTensor::from_floats(
                [[
                    [
                        [
                            [0., 147., 0., 0., 0., 150., 0.],
                            [0., 159., 0., 0., 0., 162., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 159., 0., 0., 0., 162., 0.],
                            [0., 171., 0., 0., 0., 174., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 330., 0., 0., 0., 336., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 378., 0., 0., 0., 384., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 330., 0., 0., 0., 336., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 378., 0., 0., 0., 384., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 330., 0., 0., 0., 336., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 354., 0., 0., 0., 360., 0.],
                            [0., 378., 0., 0., 0., 384., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 183., 0., 0., 0., 186., 0.],
                            [0., 195., 0., 0., 0., 198., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 195., 0., 0., 0., 198., 0.],
                            [0., 207., 0., 0., 0., 210., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                    ],
                    [
                        [
                            [0., 219., 0., 0., 0., 222., 0.],
                            [0., 231., 0., 0., 0., 234., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 231., 0., 0., 0., 234., 0.],
                            [0., 243., 0., 0., 0., 246., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 474., 0., 0., 0., 480., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 522., 0., 0., 0., 528., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 474., 0., 0., 0., 480., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 522., 0., 0., 0., 528., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 474., 0., 0., 0., 480., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 498., 0., 0., 0., 504., 0.],
                            [0., 522., 0., 0., 0., 528., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                        [
                            [0., 255., 0., 0., 0., 258., 0.],
                            [0., 267., 0., 0., 0., 270., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                            [0., 267., 0., 0., 0., 270., 0.],
                            [0., 279., 0., 0., 0., 282., 0.],
                            [0., 0., 0., 0., 0., 0., 0.],
                        ],
                    ],
                ]],
                &device,
            ),
            weight: TestTensor::from_floats(
                [
                    [
                        [
                            [
                                [0., 256., 272., 0.],
                                [0., 624., 656., 0.],
                                [0., 368., 384., 0.],
                            ],
                            [
                                [0., 424., 440., 0.],
                                [0., 960., 992., 0.],
                                [0., 536., 552., 0.],
                            ],
                        ],
                        [
                            [
                                [0., 1096., 1112., 0.],
                                [0., 2304., 2336., 0.],
                                [0., 1208., 1224., 0.],
                            ],
                            [
                                [0., 1264., 1280., 0.],
                                [0., 2640., 2672., 0.],
                                [0., 1376., 1392., 0.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [0., 256., 272., 0.],
                                [0., 624., 656., 0.],
                                [0., 368., 384., 0.],
                            ],
                            [
                                [0., 424., 440., 0.],
                                [0., 960., 992., 0.],
                                [0., 536., 552., 0.],
                            ],
                        ],
                        [
                            [
                                [0., 1096., 1112., 0.],
                                [0., 2304., 2336., 0.],
                                [0., 1208., 1224., 0.],
                            ],
                            [
                                [0., 1264., 1280., 0.],
                                [0., 2640., 2672., 0.],
                                [0., 1376., 1392., 0.],
                            ],
                        ],
                    ],
                    [
                        [
                            [
                                [0., 256., 272., 0.],
                                [0., 624., 656., 0.],
                                [0., 368., 384., 0.],
                            ],
                            [
                                [0., 424., 440., 0.],
                                [0., 960., 992., 0.],
                                [0., 536., 552., 0.],
                            ],
                        ],
                        [
                            [
                                [0., 1096., 1112., 0.],
                                [0., 2304., 2336., 0.],
                                [0., 1208., 1224., 0.],
                            ],
                            [
                                [0., 1264., 1280., 0.],
                                [0., 2640., 2672., 0.],
                                [0., 1376., 1392., 0.],
                            ],
                        ],
                    ],
                ],
                &device,
            ),
            bias: TestTensor::from_floats([10., 10., 10.], &device),
        };
        test.assert_grads(grads);
    }

    struct Conv3dTestCase {
        batch_size: usize,
        channels_in: usize,
        channels_out: usize,
        kernel_size_1: usize,
        kernel_size_2: usize,
        kernel_size_3: usize,
        padding_1: usize,
        padding_2: usize,
        padding_3: usize,
        stride_1: usize,
        stride_2: usize,
        stride_3: usize,
        dilation_1: usize,
        dilation_2: usize,
        dilation_3: usize,
        groups: usize,
        depth: usize,
        height: usize,
        width: usize,
    }

    struct Grads {
        x: TestTensor<5>,
        weight: TestTensor<5>,
        bias: TestTensor<1>,
    }

    impl Conv3dTestCase {
        fn assert_grads(self, expected_grads: Grads) {
            let shape_x = Shape::new([
                self.batch_size,
                self.channels_in,
                self.depth,
                self.height,
                self.width,
            ]);
            let shape_weight = Shape::new([
                self.channels_out,
                self.channels_in / self.groups,
                self.kernel_size_1,
                self.kernel_size_2,
                self.kernel_size_3,
            ]);
            let device = Default::default();
            let weight = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..shape_weight.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_weight)
                    .into_data(),
                &device,
            )
            .require_grad();
            let bias = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..self.channels_out as i64, &device).into_data(),
                &device,
            )
            .require_grad();
            let x = TestAutodiffTensor::from_data(
                TestTensorInt::arange(0..shape_x.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_x)
                    .into_data(),
                &device,
            )
            .require_grad();
            let output = conv3d(
                x.clone(),
                weight.clone(),
                Some(bias.clone()),
                ConvOptions::new(
                    [self.stride_1, self.stride_2, self.stride_3],
                    [self.padding_1, self.padding_2, self.padding_3],
                    [self.dilation_1, self.dilation_2, self.dilation_3],
                    self.groups,
                ),
            );
            let grads = output.backward();

            // Assert
            let x_grad_actual = x.grad(&grads).unwrap();
            let weight_grad_actual = weight.grad(&grads).unwrap();
            let bias_grad_actual = bias.grad(&grads).unwrap();

            expected_grads
                .bias
                .to_data()
                .assert_approx_eq(&bias_grad_actual.to_data(), 5);
            expected_grads
                .x
                .to_data()
                .assert_approx_eq(&x_grad_actual.to_data(), 5);
            expected_grads
                .weight
                .to_data()
                .assert_approx_eq(&weight_grad_actual.to_data(), 5);
        }
    }
}
