<html>
<head>
    <title>Projekt zespołowy</title>
    <meta charset="UTF-8">
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <ul class="navbar">
        <li id=home><a href="#">PSK</a></li>
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
        <li class="zalbtn" style="float:right";><a href="login.php">Zaloguj się</a></li>
    </ul>
    <p style="float:right;"><b>Dark mode</b></p>
    <label class="switch">
        <input type="checkbox" onclick="toggleDark()">
        <span class="slider"></span>
    </label>
    <h1>Grupa 3ID11A</h1>
    <form action="#" style="margin-left:5%;">
        <label for="tydzien"><b>Wybierz tydzień:</b></label>
            <select class="tydzien">
            <option value="1">1 tydzień 12.03-14.03</option>
            <option value="2">2 tydzień 19.03-21.03</option>
            <option value="3">3 tydzień 26.03-28.03</option>
            <option value="4">4 tydzień 09.04-11.04</option>
            <option value="5">5 tydzień 23.04-25.04</option>
            <option value="6">6 tydzień 14.05-16.05</option>
            <option value="7">7 tydzień 28.05-30.05</option>
            <option value="8">8 tydzień 11.06-13.06</option>
            <option value="9">9 tydzień 18.06-20.06</option>
            </select>
        <input type="submit" value="Wybierz">
    </form>
    <div style="height:2px; background-color:red; width:90%; margin:auto;">
        
    </div>
    <h2>Zjazd I - 1 tydzień 12.03-14.03</h2>
    <table class="plan">
        <tr class="godziny">
            <th></th>
            <th>8:00-10:00</th>
            <th>10:00-12:00</th>
            <th>12:00-14:00</th>
            <th>14:00-16:00</th>
            <th>16:00-18:00</th>
            <th>18:00-20:00</th>
        </tr>
        <tr class="piatek">
            <th>Piątek</th>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
            <td class="laboratorium"><i>Laboratorium</i><br><b>Programowanie współbieżne</b><br>dr inż Aneta Bugajska</td>
            <td class="laboratorium"><i>Laboratorium</i><br><b>Metody obliczeniowe</b>      <br>dr inż. Andrzej Kułakowski</td>
        <tr class="sobota">
            <th>Sobota</th>
            <td class="wyklad">      <i>Wykład</i>      <br><b>Grafika komputerowa</b>                    <br>dr inż Grzegorz Łukawski</td>
            <td class="cwiczenia">   <i>Ćwiczenia</i>   <br><b>Język obcy 4</b>                           <br>mgr Nina Kacperczyk</td>
            <td class="wyklad">      <i>Wykład</i>      <br><b>Projektowanie aplikacji internetowych 1</b><br>dr inż Jarosław Wikarek</td>
            <td class="laboratorium"><i>Laboratorium</i><br><b>Inżynieria programowania</b>               <br>mgr inż Paweł Pięta</td>
            <td class="laboratorium"><i>Laboratorium</i><br><b>Grafika komputerowa</b>                    <br>mgr inż. Daniel Kaczmarski</td>
            <td class="wyklad">      <i>Wykład</i>      <br><b>Metody obliczeniowe</b>                    <br>dr inż. Andrzej Kułakowski</td>
        </tr>
        <tr class="niedziela">
            <th>Niedziela</th>
            <td class="wyklad"><i>Wykład</i><br><b>Inżynieria programowania</b>               <br>dr inż. Arkadiusz Chrobot</td>
            <td class="wyklad"><i>Wykład</i><br><b>Inżynieria programowania</b>               <br>dr inż. Adam Krechowicz</td>
            <td class="wyklad"><i>Wykład</i><br><b>Programowanie współbieżne</b>              <br>dr inż. Aneta Bugajska</td>
            <td class="wyklad"><i>Wykład</i><br><b>Projektowanie aplikacji internetowych 1</b><br>dr inż. Jarosław Wikarek</td>
            <td class="wyklad"><i>Wykład</i><br><b>Grafika komputerowa</b>                    <br>dr inż. Grzegorz Łukawski</td>
            <td></td>
        </tr>
    </table>
    <br>
    <table class="legenda">
        <tr>
            <th>Legenda:</th>
            <td class="wyklad"></td>
            <td>- wykład</td>
            <td class="laboratorium"></td>
            <td>- laboratorium</td>
            <td class="cwiczenia"></td>
            <td>- cwiczenia</td>
        </tr>
    </table>
    <div class="nawigacja">
        <button class="poprzedni">&laquo; Poprzedni tydzień</button>
        <button class="nastepny">Następny tydzień &raquo;</button>
    </div>
</body>
<script>
    function toggleDark(){
        var element = document.body;
        element.classList.toggle("dark-mode");
    }
</script>
</html>