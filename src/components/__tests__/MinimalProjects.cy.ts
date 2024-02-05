import MinimalProjects from '../minimal/Projects.vue';

describe('Minimal Projects', () => {
  it('should render', () => {
    cy.mount(MinimalProjects);
    cy.get('h2').should('have.text', 'Other projects');
  });

  it('should contain 5 project items', () => {
    cy.mount(MinimalProjects);
    cy.get('section').find('th').should('have.length', 5);
  });
});
