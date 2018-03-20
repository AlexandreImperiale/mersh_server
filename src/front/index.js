import React from 'react';
import ReactDOM from 'react-dom';
import './index.css'

function Cell(props) {
  return  (
    <div>
      <textarea value={props.source} />
      <textarea value={props.outputs} />
    </div>
  );
}

class CellBoard extends React.Component {
  render() {
     const listCells = this.props.cells.map((cell) => {
       return <Cell source={cell.source} outputs={cell.outputs} />;
     });
     return <div> {listCells} </div>
  }
}

class MershNotebook extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      cells: [
        {source:"mesh = base::factory::new_mesh3d();", outputs: "Server output 0"},
        {source:"base::accessor::get_vertex3d(mesh, 0);", outputs: "Server output 1"}
      ]
    };
  }

  render() {
    return (
      <div className="notebook">
        <div className="cell-board">
          <CellBoard cells={this.state.cells} />
        </div>
      </div>
    );
  }
}

ReactDOM.render(<MershNotebook />, document.getElementById("root"));
