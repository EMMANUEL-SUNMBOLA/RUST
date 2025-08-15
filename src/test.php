<?php

$secret = rand(1, 100);
echo $secret . "\n";
$guess = readline("Input your guess ? \n");

while($guess !== $secret){

    
    if ($guess > $secret){
        echo "go lower \n";
    }elseif($guess < $secret){
        echo "go higher \n";
    }elseif($guess == $secret){
        echo "Hurray you got it 😀 \n";
        break;
    }
    
    $guess = readline("Try again ? \n");
}

// Future Improvements, add a clue button that uses the numbers API to fetch some details about the random number !!!!