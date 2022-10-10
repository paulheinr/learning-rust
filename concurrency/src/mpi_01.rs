use mpi::traits::*;

pub fn print_rank_number() {
    let universe = mpi::initialize().unwrap();
    println!("my rank number is {} of total {}", universe.world().rank(), universe.world().size())
}

pub fn print_number_per_rank() {
    let numbers = 10;
    let universe = mpi::initialize().unwrap();
    let rank = universe.world().rank();
    let size = universe.world().size();

    let numbers_per_rank = numbers / size;
    for i in numbers_per_rank * rank..numbers_per_rank * (rank + 1) {
        println!("Rank: {rank}, number: {i}");
    }
}

pub fn send_point_to_point() {
    let universe = mpi::initialize().unwrap();
    let rank = universe.world().rank();
    let size = universe.world().size();

    if size != 2 {
        panic!("Size of MPI_COMM_WORLD must be 2, but is {}!", size);
    }

    match rank {
        0 => {
            let msg = vec![4.0f64, 8.0, 15.0];
            universe.world().process_at_rank(rank + 1).send(&msg[..])
        }
        1 => {
            let (msg, status) = universe.world().any_process().receive_vec::<f64>();
            println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status)
        }
        _ => unreachable!()
    }
}

pub fn send_to_0() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    match rank {
        0 => {
            for _ in 0..size - 1 {
                let (msg, status) = world.any_process().receive::<i32>();
                println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status)
            }
        }
        r => {
            world.process_at_rank(0).send(&r)
        }
    }
}

pub fn blocking() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    let numbers_to_send = 10;

    if size != 2 {
        panic!("Must have exactly 2 ranks.")
    }

    let neighbour;
    match rank {
        0 => neighbour = 1,
        1 => neighbour = 0,
        _ => neighbour = -1
    }

    let x: Vec<i32> = (0..numbers_to_send).collect();
    world.process_at_rank(neighbour).send(&x);
    let (msg, status) = world.process_at_rank(neighbour).receive_vec::<i32>();
    println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status)
}