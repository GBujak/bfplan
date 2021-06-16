<html>
    <head>
        <title>Generuj plan</title>
        <meta charset="UTF-8">
        <link rel="stylesheet" href="styles.css">
    </head>
    <body>
    <ul class="navbar">
        <li id=home><a href="index.php">PSK</a></li>
        <li class="dropdown">
            <a href="javascript:void(0)" class="dropbtn">Znajdź grupę</a>
            <div class="dropdown-content">
                <div class="dropdown2">
                <a href="#">1 rok
                    <i class="arrow right"></i></a>
                <div class="dropdown2-content">
                    <a href="#">1ID11A</a>
                    <a href="#">1ID11B</a>
                    <a href="#">1ID12A</a>
                    <a href="#">1ID12B</a>
                </div>
                </div>
                <div class="dropdown2">
                <a href="#">2 rok
                    <i class="arrow right"></i></a>
                <div class="dropdown2-content">
                    <a href="#">2ID11A</a>
                    <a href="#">2ID11B</a>
                    <a href="#">2ID12A</a>
                    <a href="#">2ID12B</a>
                </div>
                </div>
                <div class="dropdown2">
                <a href="#">3 rok
                    <i class="arrow right"></i></a>
                <div class="dropdown2-content">
                    <a href="#">3ID11A</a>
                    <a href="#">3ID11B</a>
                    <a href="#">3ID12A</a>
                    <a href="#">3ID12B</a>
                    <a href="#">3ID13A</a>
                    <a href="#">3ID13B</a>
                </div>
                </div>
                </div>
            </div>
        </li>
        <li class="dropdown">
            <a href="#" class="dropbtn">Znajdź wykładowcę</a>
            <div class="dropdown-content">
                <a href="#">dr inż. Aneta Bugajska</a>
                <a href="#">dr inż. Arkadiusz Chrobot</a>
                <a href="#">dr inż. Grzegorz Łukawski</a>
                <a href="#">mgr Nina Kacperczyk</a>
                <a href="#">mgr inż. Daniel Kaczmarski</a>
                <a href="#">dr inż. Adam Krechowicz</a>
                <a href="#">dr inż. Andrzej Kułakowski</a>
                <a href="#">mgr inż. Paweł Pięta</a>
                <a href="#">dr inż. Jarosław Wikarek</a>
            </div>
        </li>
        <li class="dropdown">
            <a href="#" class="dropbtn">Znajdź przedmiot</a>
            <div class="dropdown-content">
                <a href="#">Inżynieria programowania</a>
                <a href="#">Język obcy 4</a>
                <a href="#">Grafika komputerowa</a>
                <a href="#">Metody obliczeniowe</a>
                <a href="#">Programowanie współbieżne</a>
                <a href="#">Projektowanie aplikacji internetowych 1</a>
            </div>
        </li>
        <li class="zalbtn" style="float:right";><a href="login.php">Wyloguj się</a></li>
    </ul>
        <div class="generator">
            <div class="genbts">
                <button class="tablinks" onclick="openG(event, 'Sala')">Dodaj salę</button>
                <button class="tablinks" onclick="openG(event, 'Grupa')">Dodaj grupę</button>
                <button class="tablinks" onclick="openG(event, 'Wykladowca')">Dodaj wykładowce</button>
                <button class="tablinks" style="float:right">Generuj</button>
            </div>
                <div class="content" id="Sala">
                    <input type="text" placeholder="Numer sali">
                    <input type="submit" value="Dodaj salę"><br>
                    <select class="wybierz_sala">
                    <option value="1">3.15d</option>
                    <option value="2">4.15d</option>
                    </select>
                    <input type="number" placeholder="Liczba miejsc">
                    <input type="submit" value="Zaktualizuj liczbę miejsc"><br>
                </div>
                <div class="content" id="Grupa">
                    <input type="text" placeholder="Numer grupy">
                    <input type="submit" value="Dodaj grupę"><br>
                    <select class="grupa_liczba_studentow">
                    <option value="1">11A</option>
                    <option value="2">11B</option>
                    </select>
                    <input type="number" placeholder="Liczba studentów">
                    <input type="submit" value="Zaktualizuj liczbę studentów"><br>
                    <select class="grupa_przedmiot">
                    <option value="1">11A</option>
                    <option value="2">11B</option>
                    </select>
                    <select class="Przedmiot">
                    <option value="1">Matematyka</option>
                    <option value="2">Informatyka</option>
                    </select>
                    <input type="submit" value="Dodaj przedimot"><br>
                </div>
                <div class="content" id="Wykladowca">
                    <input type="text" placeholder="Tytuł wykładowcy">
                    <input type="text" placeholder="Imie wykładowcy">
                    <input type="text" placeholder="Nazwisko wykładowcy">
                    <input type="text" placeholder="Tytuł wykładowcy">
                    <input type="submit" value="Dodaj wykładowcę"><br>
                    <select class="Wykładowca">
                    <option value="1">mgr. Jan Kowalski</option>
                    <option value="2">mgr. Jan Kowalski 2</option>
                    </select>
                    <select class="Przedmiot">
                    <option value="1">Matematyka</option>
                    <option value="2">Informatyka</option>
                    </select>
                    <input type="submit" value="Dodaj przedmiot wykładowcy"><br>
                </div>
            </div>
            <script>
                function openG(evt, Name) {
                  var i, tabcontent, tablinks;
                  tabcontent = document.getElementsByClassName("content");
                  for (i = 0; i < tabcontent.length; i++) {
                    tabcontent[i].style.display = "none";
                  }
                  tablinks = document.getElementsByClassName("tablinks");
                  for (i = 0; i < tablinks.length; i++) {
                    tablinks[i].className = tablinks[i].className.replace(" active", "");
                  }
                  document.getElementById(Name).style.display = "block";
                  evt.currentTarget.className += " active";
                }
                </script>
    </body>
</html>