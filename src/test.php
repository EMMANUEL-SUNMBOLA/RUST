<?php


function fibb($n){

    if($n <= 0) return 0;
    if($n === 1) return 1;

    return (fibb($n - 1) + fibb($n - 2));

}


$int = readline("what nth fibb do you want   ");

for($i = 0; $i <= $int; $i++){
    echo $i . " - " . fibb($i) . "\n";
}

// $secret = rand(1, 100);
// echo $secret . "\n";
// $guess = readline("Input your guess ? \n");

// while($guess !== $secret){

    
//     if ($guess > $secret){
//         echo "go lower \n";
//     }elseif($guess < $secret){
//         echo "go higher \n";
//     }elseif($guess == $secret){
//         echo "Hurray you got it 😀 \n";
//         break;
//     }
    
//     $guess = readline("Try again ? \n");
// }

// Future Improvements, add a clue button that uses the numbers API to fetch some details about the random number !!!!