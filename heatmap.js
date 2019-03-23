var map, heatmap, pointArray;
function initMap() {
  pointArray = new google.maps.MVCArray();
  map = new google.maps.Map(document.getElementById('map'), {
    zoom: 13,
    center: {lat: 35.227085, lng: -80.843124}, //charlotte cordinates 
    mapTypeId: 'roadmap'
  });
  generateHeatmap();
}

function generateHeatmap(){
    heatmap = new google.maps.visualization.HeatmapLayer({
        data: getPoints(),
        map: map
      });
      heatmap.set('radius', 50);
}

function toggleHeatmap() {
     heatmap.setMap(heatmap.getMap() ? null : map);
}

function createPoint(x, y){
  pointArray.push(new google.maps.LatLng(x, y));
}

function getPoints() {
    //clearPoints();
    var chkBlack = document.getElementById("black").checked;
    var chkWhite = document.getElementById("white").checked;
    var chkAsian = document.getElementById("asian").checked;
    var chkLatin = document.getElementById("latino").checked;
    var chkOther = document.getElementById("other").checked;
    var female = document.getElementById("female").checked;
    var male   = document.getElementById("male").checked;
    var both   = document.getElementById("both").checked;

    var gender = male ? 0 : female ? 1 : 2; //0 -> male, 1 -> female, 2 -> other

    console.log(gender);
    if (chkBlack) generateBlack(gender);
    if (chkWhite) generateWhite(gender);
    if (chkAsian) generateAsian(gender);
    if (chkLatin) generateAsian(gender);
    if (chkOther) generateOther(gender);
  return pointArray;
}

function clearPoints(){
    heatmap.setMap(null);
    pointArray = new google.maps.MVCArray();
    generateHeatmap();
}

function generateWhite(gender){

}
function generateBlack(gender){

}
function generateAsian(gender){
    
}
function generateLatino(gender){
    
}
function generateOther(gender){

}