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
    // var checkBox = document.getElementById("myCheck");
    // // Get the output text
    // var text = document.getElementById("text");
  
    // // If the checkbox is checked, display the output text
    // if (checkBox.checked == true){
    //   text.style.display = "block";
    // } else {
    //   text.style.display = "none";
    // }
  pointArray.push(new google.maps.LatLng(x, y));
}

function getPoints() {
  return pointArray;
}

function clearPoints(){
    heatmap.setMap(null);
    pointArray = new google.maps.MVCArray();
    generateHeatmap();
}
