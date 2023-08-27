use crate::loggingv2::ChartData;

// Unit testing my beloved

#[test]
fn buffer_flush() {
    let mut buffer = ChartData::default();
    let mut data = ChartData::default();

    let slice = [1.0, 2.0, 3.0].to_vec();

    let buffer_length = 2;
    let data_length = 5;

    for _num in 0..2 {
        buffer.push(slice.clone());
    }

    assert_eq!(buffer.0, [[1.0, 1.0], [2.0, 2.0], [3.0, 3.0]]);

    if buffer.0[0].len() >= buffer_length {
        buffer.flush_into(&mut data, data_length);
    }

    assert_eq!(data.0, [[1.0, 1.0], [2.0, 2.0], [3.0, 3.0]]);
    let empty_buffer: Vec<Vec<f32>> = vec![];
    assert_eq!(buffer.0, empty_buffer)
}

#[test]
fn buffer_trim() {
    let mut buffer = ChartData::default();
    let mut data = ChartData::default();

    let buffer_length = 2;
    let data_length = 5;

    let slice = [1.0, 2.0, 3.0].to_vec();

    for _ in 0..20 {
        buffer.push(slice.clone());
        if buffer.0[0].len() >= buffer_length {
            buffer.flush_into(&mut data, data_length);
        }
    }

    assert_eq!(
        data.0,
        vec![
            vec![1.0, 1.0, 1.0, 1.0, 1.0],
            vec![2.0, 2.0, 2.0, 2.0, 2.0],
            vec![3.0, 3.0, 3.0, 3.0, 3.0],
        ]
    );
}
